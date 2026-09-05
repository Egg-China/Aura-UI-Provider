//! Strict `aura.ui.v1` message envelope model and session state machine.

use crate::value::{decode, encode, Value};

/// Frozen UI wire schema generation.
const SCHEMA_VERSION: i64 = 1;

/// Protocol identifier announced by the launcher hello.
pub const PROTOCOL: &str = "aura.ui.v1";

/// Frozen UI wire ABI generation.
pub const ABI: i64 = 1;

/// One direction-validated protocol message.
#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    /// Launcher- or frontend-originated method invocation.
    Request {
        /// Positive direction-scoped request identifier.
        request_id: i64,
        /// Method name.
        method: String,
        /// Token-free parameter value.
        params: Value,
    },
    /// Successful reply preserving the request identifier.
    Result {
        /// Identifier copied from the answered request.
        request_id: i64,
        /// Token-free result value.
        value: Value,
    },
    /// Typed failure reply.
    Error {
        /// Identifier copied from the failed request.
        request_id: i64,
        /// Stable error code.
        code: String,
        /// Redacted diagnostic text.
        message: String,
    },
}

impl Message {
    /// Returns the request identifier of any message variant.
    pub fn request_id(&self) -> i64 {
        match self {
            Message::Request { request_id, .. }
            | Message::Result { request_id, .. }
            | Message::Error { request_id, .. } => *request_id,
        }
    }
}

/// Builds one frontend-originated request with an even identifier.
pub fn request(request_id: i64, method: &str, params: Value) -> Message {
    Message::Request {
        request_id,
        method: method.to_string(),
        params,
    }
}

/// Builds one successful reply.
pub fn result(request_id: i64, value: Value) -> Message {
    Message::Result { request_id, value }
}

/// Encodes one message into a complete frame body.
pub fn encode_message(message: &Message) -> Result<Vec<u8>, String> {
    let mut entries: Vec<(String, Value)> = vec![
        ("schemaVersion".to_string(), Value::Integer(SCHEMA_VERSION)),
        ("type".to_string(), Value::String(match message {
            Message::Request { .. } => "request".to_string(),
            Message::Result { .. } => "result".to_string(),
            Message::Error { .. } => "error".to_string(),
        })),
        (
            "requestId".to_string(),
            Value::Integer(message.request_id()),
        ),
    ];
    match message {
        Message::Request { method, params, .. } => {
            entries.push(("method".to_string(), Value::String(method.clone())));
            entries.push(("params".to_string(), params.clone()));
        }
        Message::Result { value, .. } => {
            entries.push(("value".to_string(), value.clone()));
        }
        Message::Error { code, message, .. } => {
            entries.push(("code".to_string(), Value::String(code.clone())));
            entries.push(("message".to_string(), Value::String(message.clone())));
        }
    }
    encode(&Value::Map(entries))
}

/// Decodes one frame body into a direction-validated message.
///
/// Launcher-originated requests use odd identifiers while frontend-originated
/// requests use even identifiers; replies always match their request side.
pub fn decode_message(body: &[u8], launcher_request: bool) -> Result<Message, String> {
    let value = decode(body)?;
    let entries = match &value {
        Value::Map(entries) => entries,
        _ => return Err("message envelope is not a map".to_string()),
    };
    let mut schema_version: Option<i64> = None;
    let mut kind: Option<String> = None;
    let mut request_id: Option<i64> = None;
    let mut method: Option<String> = None;
    let mut params: Option<Value> = None;
    let mut result_value: Option<Value> = None;
    let mut error_code: Option<String> = None;
    let mut error_message: Option<String> = None;
    for (key, entry) in entries {
        match key.as_str() {
            "schemaVersion" => schema_version = Some(expect_integer(entry, "schemaVersion")?),
            "type" => kind = Some(expect_string(entry, "type")?),
            "requestId" => request_id = Some(expect_integer(entry, "requestId")?),
            "method" => method = Some(expect_string(entry, "method")?),
            "params" => params = Some(entry.clone()),
            "value" => result_value = Some(entry.clone()),
            "code" => error_code = Some(expect_string(entry, "code")?),
            "message" => error_message = Some(expect_string(entry, "message")?),
            _ => return Err(format!("unsupported envelope field {key}")),
        }
    }
    if schema_version != Some(SCHEMA_VERSION) {
        return Err("unsupported schema version".to_string());
    }
    let request_id = request_id.ok_or("envelope has no requestId")?;
    if request_id <= 0 {
        return Err("request identifier must be positive".to_string());
    }
    match kind.as_deref() {
        Some("request") => {
            let launcher_origin = request_id % 2 == 1;
            if launcher_origin != launcher_request {
                return Err("request identifier does not match its direction".to_string());
            }
            Ok(Message::Request {
                request_id,
                method: method.ok_or("request has no method")?,
                params: params.ok_or("request has no params")?,
            })
        }
        Some("result") => Ok(Message::Result {
            request_id,
            value: result_value.ok_or("result has no value")?,
        }),
        Some("error") => Ok(Message::Error {
            request_id,
            code: error_code.ok_or("error has no code")?,
            message: error_message.ok_or("error has no message")?,
        }),
        _ => Err("unsupported message type".to_string()),
    }
}

/// Validates one launcher hello parameter map.
pub fn validate_hello(params: &Value) -> Result<(), String> {
    let entries = match params {
        Value::Map(entries) if entries.len() == 2 => entries,
        _ => return Err("launcher hello must contain exactly two fields".to_string()),
    };
    if entries[0].0 != "protocol"
        || entries[1].0 != "abi"
        || &entries[0].1 != &Value::String(PROTOCOL.to_string())
        || entries[1].1 != Value::Integer(ABI)
    {
        return Err("launcher hello fields do not match aura.ui.v1 ABI 1".to_string());
    }
    Ok(())
}

/// Extracts one expected integer field.
fn expect_integer(value: &Value, label: &str) -> Result<i64, String> {
    match value {
        Value::Integer(number) => Ok(*number),
        _ => Err(format!("{label} is not an integer").to_string()),
    }
}

/// Extracts one expected string field.
fn expect_string(value: &Value, label: &str) -> Result<String, String> {
    match value {
        Value::String(text) => Ok(text.clone()),
        _ => Err(format!("{label} is not a string").to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_requests_and_replies() {
        let request = request(2, "ui.ready", Value::Null);
        let encoded = encode_message(&request).unwrap();
        assert_eq!(decode_message(&encoded, false).unwrap(), request);

        let reply = result(2, Value::String("ok".to_string()));
        let encoded = encode_message(&reply).unwrap();
        assert_eq!(decode_message(&encoded, true).unwrap(), reply);
    }

    #[test]
    fn rejects_wrong_request_direction() {
        let request = request(2, "ui.ready", Value::Null);
        let encoded = encode_message(&request).unwrap();
        assert!(decode_message(&encoded, true).is_err());
    }

    #[test]
    fn validates_the_launcher_hello_contract() {
        let hello = Value::Map(vec![
            ("protocol".to_string(), Value::String(PROTOCOL.to_string())),
            ("abi".to_string(), Value::Integer(ABI)),
        ]);
        assert!(validate_hello(&hello).is_ok());
        let wrong_abi = Value::Map(vec![
            ("protocol".to_string(), Value::String(PROTOCOL.to_string())),
            ("abi".to_string(), Value::Integer(2)),
        ]);
        assert!(validate_hello(&wrong_abi).is_err());
    }
}
