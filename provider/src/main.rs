//! Aura Modern UI: a Tauri 2 window speaking the `aura.ui.v1` stdio protocol.
//!
//! The launcher spawns this process with `--stdio`. A background thread keeps
//! owning the milestone-one transport contract (hello, snapshot replace,
//! ready handshake, state pull, launcher notifications, shutdown) while the
//! main thread runs the borderless Tauri window hosting the Vue build. The
//! frontend signals readiness through the `notify_ready` IPC command; the
//! protocol thread only emits `ui.ready` after the webview mounted, keeping
//! the supervision contract honest for the JavaFX host process.

mod frame;
mod proto;
mod value;

use std::io::{self, Write};
use std::process::ExitCode;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

use proto::Message;
use value::Value as BridgeValue;

/// Next frontend request identifier; the frontend always uses even values.
const READY_REQUEST_ID: i64 = 2;

/// Identifier of the first launcher-state snapshot request.
const SNAPSHOT_REQUEST_ID: i64 = 4;

/// One launcher-pushed event awaiting pickup by the webview.
#[derive(Clone, Debug)]
struct FrontendEvent {
    kind: String,
    payload_json: String,
}

/// State shared between the protocol thread and Tauri IPC commands.
struct UiState {
    snapshot: BridgeValue,
    events: Vec<FrontendEvent>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            snapshot: BridgeValue::Null,
            events: Vec::new(),
        }
    }
}

/// Wrapper managed by Tauri holding the shared UI state.
struct SharedUiState(Arc<Mutex<UiState>>);

/// Wrapper managed by Tauri holding the webview readiness signal.
struct ReadySignal(Sender<()>);

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() != 2 || arguments[1] != "--stdio" {
        eprintln!("usage: aura-ui-provider --stdio");
        return ExitCode::from(2);
    }

    let state = Arc::new(Mutex::new(UiState::default()));
    let (ready_tx, ready_rx) = std::sync::mpsc::channel::<()>();

    let protocol_state = Arc::clone(&state);
    std::thread::spawn(move || {
        // The launcher supervises this child; both protocol termination and
        // `ui.shutdown` must tear the window down instead of orphaning the UI.
        let code = match run_protocol(ready_rx, &protocol_state) {
            Ok(()) => 0,
            Err(error) => {
                eprintln!("aura-ui-provider: {error}");
                1
            }
        };
        std::process::exit(code);
    });

    match tauri::Builder::default()
        .manage(SharedUiState(state))
        .manage(ReadySignal(ready_tx))
        .invoke_handler(tauri::generate_handler![
            notify_ready,
            get_snapshot,
            drain_events,
            request_shutdown
        ])
        .run(tauri::generate_context!())
    {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("aura-ui-provider: tauri failed: {error}");
            ExitCode::FAILURE
        }
    }
}

/// IPC: the webview reports that the Vue application finished mounting.
#[tauri::command]
fn notify_ready(ready: tauri::State<ReadySignal>) -> Result<(), String> {
    ready
        .0
        .send(())
        .map_err(|_| "the protocol thread is no longer running".to_string())
}

/// IPC: returns the last launcher snapshot as compact JSON.
#[tauri::command]
fn get_snapshot(state: tauri::State<SharedUiState>) -> String {
    state.0.lock().map(|guard| guard.snapshot.to_json()).unwrap_or_else(|_| "null".to_string())
}

/// IPC: takes every pending launcher event for the webview to replay.
#[tauri::command]
fn drain_events(state: tauri::State<SharedUiState>) -> Vec<String> {
    match state.0.lock() {
        Ok(mut guard) => guard
            .events
            .drain(..)
            .map(|event| format!("{{\"kind\":{},\"payload\":{}}}", quote(&event.kind), event.payload_json))
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// IPC: closes the window; the launcher observes the supervised child exit.
#[tauri::command]
fn request_shutdown(app: tauri::AppHandle) {
    app.exit(0);
}

/// Drives the `aura.ui.v1` conversation on the stdio transport.
fn run_protocol(ready_rx: Receiver<()>, state: &Arc<Mutex<UiState>>) -> Result<(), String> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = stdin.lock();
    let mut output = stdout.lock();

    let hello = expect_request(read_message(&mut input)?, 1, "ui.hello")?;
    proto::validate_hello(&hello)?;
    write_message(&mut output, &proto::result(1, hello))?;

    let replace = expect_request(read_message(&mut input)?, 3, "ui.snapshot.replace")?;
    eprintln!(
        "aura-ui-provider: received the initial launcher snapshot ({} wire bytes)",
        summarize(&replace)
    );
    write_message(&mut output, &proto::result(3, BridgeValue::Null))?;

    // Hold `ui.ready` until the webview mounted; the window owns this signal.
    ready_rx
        .recv()
        .map_err(|_| "the webview closed before reporting readiness".to_string())?;
    write_message(&mut output, &proto::request(READY_REQUEST_ID, "ui.ready", BridgeValue::Null))?;
    expect_result(read_message(&mut input)?, READY_REQUEST_ID)?;

    write_message(
        &mut output,
        &proto::request(SNAPSHOT_REQUEST_ID, "core.snapshot.get", BridgeValue::Null),
    )?;
    loop {
        let message = read_message(&mut input)?;
        match message {
            None => return Err("launcher stdin ended before the state snapshot".to_string()),
            Some(Message::Result { request_id, value }) if request_id == SNAPSHOT_REQUEST_ID => {
                eprintln!(
                    "aura-ui-provider: launcher state snapshot received ({} bytes of value tree)",
                    summarize(&value)
                );
                if let Ok(mut guard) = state.lock() {
                    guard.snapshot = value.clone();
                }
                break;
            }
            Some(Message::Request { request_id, method, params }) => {
                serve_launcher_request(&mut output, state, request_id, &method, &params)?;
            }
            _ => return Err("unexpected message while awaiting the launcher state".to_string()),
        }
    }

    loop {
        match read_message(&mut input)? {
            None => return Err("launcher stdin ended before ui.shutdown".to_string()),
            Some(message) => match message {
                Message::Request { request_id, method, params } => {
                    if method == "ui.shutdown" {
                        write_message(&mut output, &proto::result(request_id, BridgeValue::Null))?;
                        eprintln!("aura-ui-provider: shutdown requested by the launcher");
                        return Ok(());
                    }
                    serve_launcher_request(&mut output, state, request_id, &method, &params)?;
                }
                _ => return Err("unexpected unsolicited launcher reply".to_string()),
            },
        }
    }
}

/// Serves one launcher-originated request and mirrors UI-affecting calls.
fn serve_launcher_request(
    output: &mut impl Write,
    state: &Arc<Mutex<UiState>>,
    request_id: i64,
    method: &str,
    params: &BridgeValue,
) -> Result<(), String> {
    match method {
        "ui.navigate" => {
            eprintln!("aura-ui-provider: navigate requested");
            if let Ok(mut guard) = state.lock() {
                guard.events.push(FrontendEvent {
                    kind: "navigate".to_string(),
                    payload_json: params.to_json(),
                });
            }
            write_message(output, &proto::result(request_id, params.clone()))
        }
        "ui.notify" => {
            eprintln!("aura-ui-provider: notification received");
            if let Ok(mut guard) = state.lock() {
                guard.events.push(FrontendEvent {
                    kind: "notify".to_string(),
                    payload_json: params.to_json(),
                });
            }
            write_message(output, &proto::result(request_id, BridgeValue::Null))
        }
        _ => write_message(
            output,
            &Message::Error {
                request_id,
                code: "UNSUPPORTED".to_string(),
                message: format!("unsupported launcher method {method}"),
            },
        ),
    }
}

/// Reads one framed message from the launcher.
fn read_message(input: &mut impl io::Read) -> Result<Option<Message>, String> {
    match frame::read_frame(input)? {
        None => Ok(None),
        Some(body) => proto::decode_message(&body, true).map(Some),
    }
}

/// Writes one framed message to the launcher.
fn write_message(output: &mut impl Write, message: &Message) -> Result<(), String> {
    let body = proto::encode_message(message)?;
    frame::write_frame(output, &body)?;
    output
        .flush()
        .map_err(|error| format!("failed flushing stdout: {error}"))
}

/// Requires one exact launcher request.
fn expect_request(message: Option<Message>, request_id: i64, method: &str) -> Result<BridgeValue, String> {
    match message {
        Some(Message::Request {
            request_id: actual_id,
            method: actual_method,
            params,
        }) if actual_id == request_id && actual_method == method => Ok(params),
        _ => Err(format!("expected launcher request {request_id} {method}")),
    }
}

/// Requires one exact launcher reply.
fn expect_result(message: Option<Message>, request_id: i64) -> Result<BridgeValue, String> {
    match message {
        Some(Message::Result { request_id: actual_id, value }) if actual_id == request_id => Ok(value),
        _ => Err(format!("expected launcher result {request_id}")),
    }
}

/// Returns a compact human-readable size of a value tree.
fn summarize(value: &BridgeValue) -> usize {
    value::encode(value).map(|encoded| encoded.len()).unwrap_or(0)
}

/// Quotes one identifier for the hand-rolled event JSON envelope.
fn quote(value: &str) -> String {
    BridgeValue::String(value.to_string()).to_json()
}
