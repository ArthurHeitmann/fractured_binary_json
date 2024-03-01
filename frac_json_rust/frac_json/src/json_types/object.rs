use serde_json::{Map, Value};

use crate::{byte_stream::ByteStream, keys_table::KeysTables};

use super::value::{read_value, write_value};

pub fn read_object(
    bytes: &mut ByteStream,
    length: usize,
    keys_table: &KeysTables,
) -> Result<Value, String> {
    if length == 0 {
        return Ok(Value::Object(Map::new()));
    }
    let mut map = Map::with_capacity(length);
    for _ in 0..length {
        let key_index = bytes.read_u16()?;
        let key = keys_table.lookup_index(key_index as usize)?;
        let value = read_value(bytes, keys_table)?;
        map.insert(key.clone(), value);
    }
    return Ok(Value::Object(map));
}

pub fn write_object(
    object: &Map<String, Value>,
    bytes: &mut ByteStream,
    keys_table: &mut KeysTables,
) -> Result<(), String> {
    for (key, value) in object {
        let key_index = keys_table.lookup_key_or_insert_locally(key)?;
        bytes.write_u16(key_index as u16)?;
        write_value(value, bytes, keys_table)?;
    }
    Ok(())
}
