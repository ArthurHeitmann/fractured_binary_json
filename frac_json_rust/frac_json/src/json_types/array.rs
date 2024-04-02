use serde_json::Value;

use crate::{
    byte_stream::{ByteReader, ByteWriter},
    keys_table::{DecodeKeysTables, EncodeKeysTables},
};

use super::value::{read_value, write_value};

pub fn read_array(
    bytes: &mut ByteReader,
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

pub fn write_array<'a, 'b: 'a, W: ByteWriter>(
    array: &'b Vec<Value>,
    bytes: &mut W,
    keys_table: &mut EncodeKeysTables<'a>,
) -> Result<(), String> {
    for value in array {
        write_value(value, bytes, keys_table)?;
    }
    Ok(())
}
