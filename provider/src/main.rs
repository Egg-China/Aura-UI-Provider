//! Aura Modern UI: a Tauri 2 window speaking the `aura.ui.v1` stdio protocol.
//!
//! The launcher spawns this process with `--stdio`. A background thread owns
//! the `aura.ui.v1` transport contract (hello, snapshot replace, ready
//! handshake, state pull, launcher notifications, shutdown) while the main
//! thread runs the borderless Tauri window hosting the Vue build. Webview
//! commands beyond snapshot reading travel the same protocol: the Tauri
//! `frontend_request` command queues an even-identifier `core.*` request,
//! the transport thread writes it, and the launcher reply wakes the caller.

mod frame;
mod proto;
mod value;

use std::collections::HashMap;
use std::io::{self, Write};
use std::process::ExitCode;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

use proto::Message;
use value::Value as BridgeValue;

/// First dynamic frontend request identifier after the fixed handshake ones.
const FIRST_DYNAMIC_REQUEST_ID: i64 = 6;

/// Identifier of the handshake state snapshot request.
const SNAPSHOT_REQUEST_ID: i64 = 4;

/// Identifier of the fixed readiness notification.
const READY_REQUEST_ID: i64 = 2;

/// One launcher-pushed event awaiting pickup by the webview.
#[derive(Clone, Debug)]
struct FrontendEvent {
    kind: String,
    payload_json: String,
}

/// State shared between the transport thread and Tauri IPC commands.
struct UiState {
    snapshot: BridgeValue,
    events: Vec<FrontendEvent>,
    pending: HashMap<i64, Sender<RequestOutcome>>,
    next_request_id: i64,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            snapshot: BridgeValue::Null,
            events: Vec::new(),
            pending: HashMap::new(),
            next_request_id: FIRST_DYNAMIC_REQUEST_ID,
        }
    }
}

/// Wrapper managed by Tauri holding the shared UI state.
struct SharedUiState(Arc<Mutex<UiState>>);

/// Wrapper managed by Tauri holding the webview readiness signal.
struct ReadySignal(Sender<()>);

/// Wrapper managed by Tauri holding the shared transport queue sender.
struct RequestChannel(Sender<Incoming>);

/// Messages interleaved onto the single transport work queue.
enum Incoming {
    /// One decoded launcher message from the reader half of stdin.
    FromLauncher(Message),
    /// One webview-originated request awaiting its launcher reply.
    Request {
        id: i64,
        method: String,
        params: BridgeValue,
        reply: Sender<RequestOutcome>,
    },
}

/// Terminal outcome of one webview-originated request.
enum RequestOutcome {
    /// Successful launcher result value.
    Result(BridgeValue),
    /// Typed launcher failure.
    Error(String, String),
}

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() != 2 || arguments[1] != "--stdio" {
        eprintln!("usage: aura-ui-provider --stdio");
        return ExitCode::from(2);
    }

    let state = Arc::new(Mutex::new(UiState::default()));
    let (ready_tx, ready_rx) = std::sync::mpsc::channel::<()>();
    let page_ready_tx = ready_tx.clone();
    let (incoming_tx, incoming_rx) = std::sync::mpsc::channel::<Incoming>();

    let protocol_state = Arc::clone(&state);
    let reader_tx = incoming_tx.clone();
    std::thread::spawn(move || {
        // The launcher supervises this child; both protocol termination and
        // `ui.shutdown` must tear the window down instead of orphaning the UI.
        let code = match run_protocol(ready_rx, incoming_rx, reader_tx, &protocol_state) {
            Ok(()) => 0,
            Err(error) => {
                eprintln!("aura-ui-provider: {error}");
                1
            }
        };
        std::process::exit(code);
    });

    match tauri::Builder::default()
        .on_page_load(move |_, payload| {
            if payload.event() == tauri::webview::PageLoadEvent::Finished {
                // The native page-load event is the reliable cross-platform readiness
                // signal; the webview IPC call remains a redundant fallback.
                let _ = page_ready_tx.send(());
            }
        })
        .manage(SharedUiState(state))
        .manage(ReadySignal(ready_tx))
        .manage(RequestChannel(incoming_tx))
        .invoke_handler(tauri::generate_handler![
            notify_ready,
            get_snapshot,
            drain_events,
            frontend_request,
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

/// IPC: forwards one `core.*` request to the launcher and awaits its reply.
#[tauri::command]
fn frontend_request(
    channel: tauri::State<RequestChannel>,
    state: tauri::State<SharedUiState>,
    method: String,
    params_json: String,
) -> Result<String, String> {
    let params = bridge_from_json(&params_json)?;
    let (reply_tx, reply_rx) = std::sync::mpsc::channel::<RequestOutcome>();
    let request_id = {
        let mut guard = state.0.lock().map_err(|_| "the UI state lock is poisoned".to_string())?;
        let request_id = guard.next_request_id;
        guard.next_request_id += 2;
        request_id
    };
    channel
        .0
        .send(Incoming::Request {
            id: request_id,
            method,
            params,
            reply: reply_tx,
        })
        .map_err(|_| "the protocol thread is no longer running".to_string())?;
    match reply_rx.recv() {
        Ok(RequestOutcome::Result(value)) => Ok(value.to_json()),
        Ok(RequestOutcome::Error(code, message)) => Err(format!("{code}: {message}")),
        Err(_) => Err("the protocol closed before the launcher replied".to_string()),
    }
}

/// IPC: closes the window; the launcher observes the supervised child exit.
#[tauri::command]
fn request_shutdown(app: tauri::AppHandle) {
    app.exit(0);
}

/// Converts one JSON document into the protocol Bridge value tree.
fn bridge_from_json(document: &str) -> Result<BridgeValue, String> {
    let parsed: serde_json::Value =
        serde_json::from_str(document).map_err(|error| format!("invalid command parameters: {error}"))?;
    json_to_bridge(&parsed)
}

/// Maps one parsed JSON node onto the closed Bridge value model.
fn json_to_bridge(node: &serde_json::Value) -> Result<BridgeValue, String> {
    match node {
        serde_json::Value::Null => Ok(BridgeValue::Null),
        serde_json::Value::Bool(value) => Ok(BridgeValue::Boolean(*value)),
        serde_json::Value::Number(value) => {
            if let Some(integer) = value.as_i64() {
                Ok(BridgeValue::Integer(integer))
            } else {
                value
                    .as_f64()
                    .filter(|float| float.is_finite())
                    .map(BridgeValue::Float)
                    .ok_or_else(|| "non-finite numbers are not representable".to_string())
            }
        }
        serde_json::Value::String(value) => Ok(BridgeValue::String(value.clone())),
        serde_json::Value::Array(values) => {
            let mapped = values.iter().map(json_to_bridge).collect::<Result<Vec<_>, _>>()?;
            Ok(BridgeValue::Array(mapped))
        }
        serde_json::Value::Object(entries) => {
            let mapped = entries
                .iter()
                .map(|(key, value)| Ok((key.clone(), json_to_bridge(value)?)))
                .collect::<Result<Vec<_>, String>>()?;
            Ok(BridgeValue::Map(mapped))
        }
    }
}

/// Drives the `aura.ui.v1` conversation on the stdio transport.
fn run_protocol(
    ready_rx: Receiver<()>,
    incoming_rx: Receiver<Incoming>,
    reader_tx: Sender<Incoming>,
    state: &Arc<Mutex<UiState>>,
) -> Result<(), String> {
    // Owned std handles keep the reader half movable into its thread; both
    // types buffer internally, so no explicit BufReader wrapper is needed.
    let mut input = io::stdin();
    let mut output = io::stdout();

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
        match read_message(&mut input)? {
            None => return Err("launcher stdin ended before the state snapshot".to_string()),
            Some(Message::Result { request_id, value }) if request_id == SNAPSHOT_REQUEST_ID => {
                eprintln!(
                    "aura-ui-provider: launcher state snapshot received ({} bytes of value tree)",
                    summarize(&value)
                );
                if let Ok(mut guard) = state.lock() {
                    guard.snapshot = value;
                }
                break;
            }
            Some(Message::Request { request_id, method, params }) => {
                serve_launcher_request(&mut output, state, request_id, &method, &params)?;
            }
            _ => return Err("unexpected message while awaiting the launcher state".to_string()),
        }
    }

    // The reader half now owns stdin; launcher traffic and webview requests
    // interleave through the single merged queue consumed below.
    std::thread::spawn(move || loop {
        match read_message(&mut input) {
            Ok(Some(message)) => {
                if reader_tx.send(Incoming::FromLauncher(message)).is_err() {
                    break;
                }
            }
            Ok(None) | Err(_) => break,
        }
    });

    loop {
        match incoming_rx.recv() {
            Err(_) => return Err("launcher stdin ended before ui.shutdown".to_string()),
            Ok(Incoming::FromLauncher(message)) => match message {
                Message::Request { request_id, method, params } => {
                    if method == "ui.shutdown" {
                        write_message(&mut output, &proto::result(request_id, BridgeValue::Null))?;
                        eprintln!("aura-ui-provider: shutdown requested by the launcher");
                        return Ok(());
                    }
                    serve_launcher_request(&mut output, state, request_id, &method, &params)?;
                }
                Message::Result { request_id, value } => {
                    complete_request(state, request_id, RequestOutcome::Result(value))
                }
                Message::Error { request_id, code, message } => {
                    complete_request(state, request_id, RequestOutcome::Error(code, message))
                }
            },
            Ok(Incoming::Request { id, method, params, reply }) => {
                if let Ok(mut guard) = state.lock() {
                    guard.pending.insert(id, reply);
                }
                write_message(&mut output, &proto::request(id, &method, params))?;
            }
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

/// Delivers one launcher reply to the waiting webview request, if any.
fn complete_request(state: &Arc<Mutex<UiState>>, request_id: i64, outcome: RequestOutcome) {
    if let Ok(mut guard) = state.lock() {
        if let Some(reply) = guard.pending.remove(&request_id) {
            let _ = reply.send(outcome);
        }
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
