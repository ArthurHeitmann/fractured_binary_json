use serde_json::Value;

use crate::{
    byte_stream::ByteStream,
    keys_table::{DecodeKeysTables, EncodeKeysTables},
};

use super::value::{read_value, write_value};

pub fn read_array(
    bytes: &mut ByteStream,
    length: usize,
    keys_table: &mut DecodeKeysTables,
) -> Result<Value, String> {
    if length == 0 {
        return Ok(Value::Array(Vec::new()));
    }
    let mut array = Vec::with_capacity(length);
    for _ in 0..length {
        array.push(read_value(bytes, keys_table)?);
    }
    return Ok(Value::Array(array));
}

pub fn write_array<'a, 'b: 'a>(
    array: &'b Vec<Value>,
    bytes: &mut ByteStream,
    keys_table: &mut EncodeKeysTables<'a>,
) -> Result<(), String> {
    for value in array {
        write_value(value, bytes, keys_table)?;
    }
    Ok(())
}
