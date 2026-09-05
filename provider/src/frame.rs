//! Four-byte big-endian length-prefixed `aura.ui.v1` frame transport.

use std::io::{Read, Write};

/// Maximum accepted frame body size matching Aura Launcher.
pub const MAX_FRAME_BYTES: usize = 16 * 1024 * 1024;

/// Reads one complete frame body from the stream.
///
/// Returns `None` only for a clean EOF before any header byte.
pub fn read_frame(input: &mut impl Read) -> Result<Option<Vec<u8>>, String> {
    let mut header = [0u8; 4];
    match input.read_exact(&mut header[..1]) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(error) => return Err(format!("failed reading the frame header: {error}")),
    }
    input
        .read_exact(&mut header[1..])
        .map_err(|error| format!("failed reading the frame header: {error}"))?;
    let length = u32::from_be_bytes(header) as usize;
    if length == 0 || length > MAX_FRAME_BYTES {
        return Err("frame length is outside bounds".to_string());
    }
    let mut body = vec![0u8; length];
    input
        .read_exact(&mut body)
        .map_err(|error| format!("failed reading the frame body: {error}"))?;
    Ok(Some(body))
}

/// Writes one complete length-prefixed frame body to the stream.
pub fn write_frame(output: &mut impl Write, body: &[u8]) -> Result<(), String> {
    if body.is_empty() || body.len() > MAX_FRAME_BYTES {
        return Err("frame length is outside bounds".to_string());
    }
    output
        .write_all(&(body.len() as u32).to_be_bytes())
        .and_then(|_| output.write_all(body))
        .map_err(|error| format!("failed writing the frame: {error}"))
}
