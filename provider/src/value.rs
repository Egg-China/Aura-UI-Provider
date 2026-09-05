//! Canonical tagged MessagePack Bridge Value v1 codec for `aura.ui.v1`.
//!
//! Every value is encoded as `[0x92, tag, payload]`. Strings use `str32` (`0xdb`),
//! bytes use `bin32` (`0xc6`), containers use `array32` (`0xdd`), integers use
//! `int64` (`0xd3`), floats use `f64` (`0xcb`), and map keys are bare `str32`
//! entries while map values stay fully tagged. The byte layout must remain
//! byte-for-byte compatible with `RuntimeBridgeWireCodec` in Aura Launcher.

/// A decoded Bridge Value tree.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Null singleton.
    Null,
    /// Boolean scalar.
    Boolean(bool),
    /// Signed 64-bit integer scalar.
    Integer(i64),
    /// IEEE-754 binary64 scalar.
    Float(f64),
    /// UTF-8 string scalar.
    String(String),
    /// Opaque byte scalar.
    Bytes(Vec<u8>),
    /// Ordered value array.
    Array(Vec<Value>),
    /// Insertion-ordered string map.
    Map(Vec<(String, Value)>),
}

/// Wire tag ordinals matching `BridgeValue.Tag` in Aura Launcher.
mod tag {
    pub const NULL: u8 = 0;
    pub const BOOLEAN: u8 = 1;
    pub const INTEGER: u8 = 2;
    pub const FLOAT: u8 = 3;
    pub const STRING: u8 = 4;
    pub const BYTES: u8 = 5;
    pub const ARRAY: u8 = 6;
    pub const MAP: u8 = 7;
}

/// Maximum zero-based recursive depth accepted by the protocol.
const MAX_DEPTH: usize = 63;

/// Encodes one value into the canonical wire representation.
pub fn encode(value: &Value) -> Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(64);
    encode_into(&mut out, value, 0)?;
    Ok(out)
}

/// Writes one tagged value recursively.
fn encode_into(out: &mut Vec<u8>, value: &Value, depth: usize) -> Result<(), String> {
    if depth > MAX_DEPTH {
        return Err("value depth exceeds the wire limit".to_string());
    }
    out.push(0x92);
    match value {
        Value::Null => {
            out.push(tag::NULL);
            out.push(0xc0);
        }
        Value::Boolean(bit) => {
            out.push(tag::BOOLEAN);
            out.push(if *bit { 0xc3 } else { 0xc2 });
        }
        Value::Integer(number) => {
            out.push(tag::INTEGER);
            out.push(0xd3);
            out.extend_from_slice(&number.to_be_bytes());
        }
        Value::Float(number) => {
            out.push(tag::FLOAT);
            out.push(0xcb);
            out.extend_from_slice(&number.to_be_bytes());
        }
        Value::String(text) => {
            out.push(tag::STRING);
            write_string(out, text)?;
        }
        Value::Bytes(bytes) => {
            out.push(tag::BYTES);
            out.push(0xc6);
            out.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
            out.extend_from_slice(bytes);
        }
        Value::Array(entries) => {
            out.push(tag::ARRAY);
            write_container(out, entries.len())?;
            for entry in entries {
                encode_into(out, entry, depth + 1)?;
            }
        }
        Value::Map(entries) => {
            out.push(tag::MAP);
            write_container(out, entries.len())?;
            for (key, entry) in entries {
                out.push(0x92);
                write_string(out, key)?;
                encode_into(out, entry, depth + 1)?;
            }
        }
    }
    Ok(())
}

/// Writes one `str32` UTF-8 string payload.
fn write_string(out: &mut Vec<u8>, text: &str) -> Result<(), String> {
    let encoded = text.as_bytes();
    if encoded.len() > u32::MAX as usize {
        return Err("string exceeds the wire limit".to_string());
    }
    out.push(0xdb);
    out.extend_from_slice(&(encoded.len() as u32).to_be_bytes());
    out.extend_from_slice(encoded);
    Ok(())
}

/// Writes one `array32` container length payload.
fn write_container(out: &mut Vec<u8>, length: usize) -> Result<(), String> {
    if length > u32::MAX as usize {
        return Err("container exceeds the wire limit".to_string());
    }
    out.push(0xdd);
    out.extend_from_slice(&(length as u32).to_be_bytes());
    Ok(())
}

/// Decodes exactly one value and rejects trailing bytes.
pub fn decode(input: &[u8]) -> Result<Value, String> {
    let mut cursor = 0usize;
    let value = decode_at(input, &mut cursor, 0)?;
    if cursor != input.len() {
        return Err("trailing bytes after the root value".to_string());
    }
    Ok(value)
}

/// Reads one tagged value at the cursor.
fn decode_at(input: &[u8], cursor: &mut usize, depth: usize) -> Result<Value, String> {
    if depth > MAX_DEPTH {
        return Err("value depth exceeds the wire limit".to_string());
    }
    require_byte(input, cursor, 0x92, "value header")?;
    let kind = take_byte(input, cursor, "value tag")?;
    match kind {
        tag::NULL => {
            require_byte(input, cursor, 0xc0, "null payload")?;
            Ok(Value::Null)
        }
        tag::BOOLEAN => {
            let bit = take_byte(input, cursor, "boolean payload")?;
            match bit {
                0xc3 => Ok(Value::Boolean(true)),
                0xc2 => Ok(Value::Boolean(false)),
                _ => Err("boolean payload is not canonical".to_string()),
            }
        }
        tag::INTEGER => {
            require_byte(input, cursor, 0xd3, "integer marker")?;
            Ok(Value::Integer(read_i64(input, cursor)?))
        }
        tag::FLOAT => {
            require_byte(input, cursor, 0xcb, "float marker")?;
            Ok(Value::Float(f64::from_bits(read_i64(input, cursor)? as u64)))
        }
        tag::STRING => Ok(Value::String(read_string(input, cursor)?)),
        tag::BYTES => {
            require_byte(input, cursor, 0xc6, "bytes marker")?;
            let length = read_u32(input, cursor, "bytes length")? as usize;
            if *cursor + length > input.len() {
                return Err("bytes payload is truncated".to_string());
            }
            let bytes = input[*cursor..*cursor + length].to_vec();
            *cursor += length;
            Ok(Value::Bytes(bytes))
        }
        tag::ARRAY => {
            let length = read_container_length(input, cursor, "array length")?;
            let mut entries = Vec::with_capacity(length);
            for _ in 0..length {
                entries.push(decode_at(input, cursor, depth + 1)?);
            }
            Ok(Value::Array(entries))
        }
        tag::MAP => {
            let length = read_container_length(input, cursor, "map length")?;
            let mut entries = Vec::with_capacity(length);
            for _ in 0..length {
                require_byte(input, cursor, 0x92, "map entry header")?;
                let key = read_string(input, cursor)?;
                let value = decode_at(input, cursor, depth + 1)?;
                entries.push((key, value));
            }
            Ok(Value::Map(entries))
        }
        _ => Err(format!("unsupported value tag {kind}")),
    }
}

/// Requires one exact marker byte.
fn require_byte(input: &[u8], cursor: &mut usize, expected: u8, label: &str) -> Result<(), String> {
    let actual = take_byte(input, cursor, label)?;
    if actual != expected {
        return Err(format!("{label} is not canonical"));
    }
    Ok(())
}

/// Takes one byte at the cursor.
fn take_byte(input: &[u8], cursor: &mut usize, label: &str) -> Result<u8, String> {
    if *cursor >= input.len() {
        return Err(format!("{label} is truncated"));
    }
    let byte = input[*cursor];
    *cursor += 1;
    Ok(byte)
}

/// Reads one big-endian signed 64-bit integer.
fn read_i64(input: &[u8], cursor: &mut usize) -> Result<i64, String> {
    if *cursor + 8 > input.len() {
        return Err("integer payload is truncated".to_string());
    }
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&input[*cursor..*cursor + 8]);
    *cursor += 8;
    Ok(i64::from_be_bytes(bytes))
}

/// Reads one big-endian unsigned 32-bit integer.
fn read_u32(input: &[u8], cursor: &mut usize, label: &str) -> Result<u32, String> {
    if *cursor + 4 > input.len() {
        return Err(format!("{label} is truncated"));
    }
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&input[*cursor..*cursor + 4]);
    *cursor += 4;
    Ok(u32::from_be_bytes(bytes))
}

/// Reads one canonical `array32` container length.
fn read_container_length(input: &[u8], cursor: &mut usize, label: &str) -> Result<usize, String> {
    require_byte(input, cursor, 0xdd, "container marker")?;
    Ok(read_u32(input, cursor, label)? as usize)
}

/// Reads one canonical `str32` UTF-8 string.
fn read_string(input: &[u8], cursor: &mut usize) -> Result<String, String> {
    require_byte(input, cursor, 0xdb, "string marker")?;
    let length = read_u32(input, cursor, "string length")? as usize;
    if *cursor + length > input.len() {
        return Err("string payload is truncated".to_string());
    }
    let text = std::str::from_utf8(&input[*cursor..*cursor + length])
        .map_err(|_| "string payload is not UTF-8".to_string())?
        .to_string();
    *cursor += length;
    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_every_scalar() {
        let values = vec![
            Value::Null,
            Value::Boolean(true),
            Value::Boolean(false),
            Value::Integer(-42),
            Value::Float(2.5),
            Value::String("aura".to_string()),
            Value::Bytes(vec![1, 2, 3]),
        ];
        for value in values {
            assert_eq!(decode(&encode(&value).unwrap()).unwrap(), value);
        }
    }

    #[test]
    fn round_trips_nested_containers() {
        let value = Value::Map(vec![
            ("protocol".to_string(), Value::String("aura.ui.v1".to_string())),
            ("abi".to_string(), Value::Integer(1)),
            ("list".to_string(), Value::Array(vec![Value::Null, Value::Boolean(false)])),
        ]);
        let encoded = encode(&value).unwrap();
        assert_eq!(decode(&encoded).unwrap(), value);
    }

    #[test]
    fn rejects_trailing_bytes() {
        let mut encoded = encode(&Value::Null).unwrap();
        encoded.push(0x00);
        assert!(decode(&encoded).is_err());
    }
}
