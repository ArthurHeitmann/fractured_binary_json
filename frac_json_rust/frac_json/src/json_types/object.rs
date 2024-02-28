use serde_json::{Map, Value};

use crate::{byte_stream::ByteStream, keys_table::KeysTables};

use super::{
    data_type::{DataType, DataTypes},
    value::{read_value, write_value},
};

pub fn read_object_from_data_type(
    bytes: &mut ByteStream,
    data_type: DataType,
    keys_table: &KeysTables,
) -> Result<Value, String> {
    let length = data_type.read_type_size(bytes)?;
    if length == 0 {
        return Ok(Value::Object(Map::new()));
    }
    let mut map: Map<String, Value> = Map::new();
    for _ in 0..length {
        let key_index = bytes.read_u16()?;
        let key = keys_table.lookup_index(key_index as usize)?;
        let value = read_value(bytes, keys_table)?;
        map.insert(key.clone(), value);
    }
    return Ok(Value::Object(map));
}

const OBJECT_SIZE_CHARS: [u8; 4] = [
    DataTypes::EMPTY_OBJECT.char,
    DataTypes::SMALL_OBJECT.char,
    DataTypes::BIG_OBJECT.char,
    DataTypes::LONG_OBJECT.char,
];
pub fn write_object(
    object: &Map<String, Value>,
    bytes: &mut ByteStream,
    keys_table: &mut KeysTables,
) -> Result<(), String> {
    DataType::write_char_and_size_class(object.len(), &OBJECT_SIZE_CHARS, bytes)?;
    if object.is_empty() {
        return Ok(());
    }
    for (key, value) in object {
        let key_index = keys_table.lookup_key_or_insert_locally(key)?;
        bytes.write_u16(key_index as u16)?;
        write_value(value, bytes, keys_table)?;
    }
    Ok(())
}
