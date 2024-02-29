use serde_json::Value;

use crate::byte_stream::ByteStream;

use super::data_type::{DataType, DataTypes};

pub fn read_string_from_data_type(
    bytes: &mut ByteStream,
    data_type: DataType,
) -> Result<Value, String> {
    let length = data_type.read_type_size(bytes)?;
    if length == 0 {
        return Ok(Value::String("".to_string()));
    }
    return Ok(Value::String(bytes.read_string(length)?));
}

const STRING_SIZE_CHARS: [u8; 4] = [
    DataTypes::EMPTY_STRING.char,
    DataTypes::SMALL_STRING.char,
    DataTypes::BIG_STRING.char,
    DataTypes::LONG_STRING.char,
];
pub fn write_string(string: &String, bytes: &mut ByteStream) -> Result<(), String> {
    DataType::write_char_and_size_class(string.len(), &STRING_SIZE_CHARS, bytes)?;
    if string.is_empty() {
        return Ok(());
    }
    bytes.write_string(string)
}
