//! Isolated Aura UI frontend protocol host.
//!
//! Milestone one implements the complete `aura.ui.v1` transport contract: it
//! answers `ui.hello`, stores `ui.snapshot.replace`, reports `ui.ready`, serves
//! launcher notifications, requests one launcher snapshot through
//! `core.snapshot.get`, and exits cleanly after `ui.shutdown`. The Tauri 2 +
//! Vue window attaches on top of this supervision-safe process boundary.

mod frame;
mod proto;
mod value;

use std::io::{self, Write};
use std::process::ExitCode;

use proto::Message;
use value::Value as BridgeValue;

/// Next frontend request identifier; the frontend always uses even values.
const READY_REQUEST_ID: i64 = 2;

/// Identifier of the first launcher-state snapshot request.
const SNAPSHOT_REQUEST_ID: i64 = 4;

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() != 2 || arguments[1] != "--stdio" {
        eprintln!("usage: aura-ui-provider --stdio");
        return ExitCode::from(2);
    }
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("aura-ui-provider: {error}");
            ExitCode::FAILURE
        }
    }
}

/// Drives the complete milestone-one protocol conversation.
fn run() -> Result<(), String> {
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
                break;
            }
            Some(Message::Request { request_id, method, params }) => {
                serve_launcher_request(&mut output, request_id, &method, &params)?;
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
                    serve_launcher_request(&mut output, request_id, &method, &params)?;
                }
                _ => return Err("unexpected unsolicited launcher reply".to_string()),
            },
        }
    }
}

/// Serves one launcher-originated request while a frontend request may be pending.
fn serve_launcher_request(
    output: &mut impl Write,
    request_id: i64,
    method: &str,
    params: &BridgeValue,
) -> Result<(), String> {
    match method {
        "ui.navigate" => {
            eprintln!("aura-ui-provider: navigate requested");
            write_message(output, &proto::result(request_id, params.clone()))
        }
        "ui.notify" => {
            eprintln!("aura-ui-provider: notification received");
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
