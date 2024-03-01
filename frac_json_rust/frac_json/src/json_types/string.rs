use serde_json::Value;

use crate::byte_stream::ByteStream;

pub fn read_string(bytes: &mut ByteStream, length: usize) -> Result<Value, String> {
    if length == 0 {
        return Ok(Value::String("".to_string()));
    }
    return Ok(Value::String(bytes.read_string(length)?));
}

pub fn write_string(string: &String, bytes: &mut ByteStream) -> Result<(), String> {
    if string.is_empty() {
        return Ok(());
    }
    bytes.write_string(string)
}
