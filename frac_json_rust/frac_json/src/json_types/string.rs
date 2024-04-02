use serde_json::Value;

use crate::byte_stream::{ByteReader, ByteWriter};

pub fn read_string(bytes: &mut ByteReader, length: usize) -> Result<Value, String> {
    if length == 0 {
        return Ok(Value::String("".to_string()));
    }
    return Ok(Value::String(bytes.read_string(length)?));
}

pub fn write_string<W: ByteWriter>(string: &String, bytes: &mut W) {
    if !string.is_empty() {
        bytes.write_string(string);
    }
}
