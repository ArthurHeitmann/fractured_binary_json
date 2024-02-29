use serde_json::Value;

use crate::{byte_stream::ByteStream, keys_table::KeysTables};

use super::{
    data_type::{DataType, DataTypes},
    value::{read_value, write_value},
};

pub fn read_array_from_data_type(
    bytes: &mut ByteStream,
    data_type: DataType,
    keys_table: &KeysTables,
) -> Result<Value, String> {
    let length = data_type.read_type_size(bytes)?;
    if length == 0 {
        return Ok(Value::Array(Vec::new()));
    }
    let mut array = Vec::with_capacity(length);
    for _ in 0..length {
        array.push(read_value(bytes, keys_table)?);
    }
    return Ok(Value::Array(array));
}

const ARRAY_SIZE_CHARS: [u8; 4] = [
    DataTypes::EMPTY_ARRAY.char,
    DataTypes::SMALL_ARRAY.char,
    DataTypes::BIG_ARRAY.char,
    DataTypes::LONG_ARRAY.char,
];
pub fn write_array(
    array: &Vec<Value>,
    bytes: &mut ByteStream,
    keys_table: &mut KeysTables,
) -> Result<(), String> {
    DataType::write_char_and_size_class(array.len(), &ARRAY_SIZE_CHARS, bytes)?;
    if array.is_empty() {
        return Ok(());
    }
    for value in array {
        write_value(value, bytes, keys_table)?;
    }
    Ok(())
}
