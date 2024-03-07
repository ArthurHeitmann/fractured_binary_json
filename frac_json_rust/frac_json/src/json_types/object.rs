use serde_json::{Map, Value};

use crate::{byte_stream::ByteStream, keys_table::KeysTables};

use super::value::{read_value, write_value};


const IMMEDIATE_TINY_START: u8 = 0x03;
const BACK_REFERENCE_TINY_START: u8 = 0x57;
const GLOBAL_INDEX_TINY_START: u8 = 0xAB;
const RESERVED: u8 = 0xFF;
const IMMEDIATE_MAX: u8 = BACK_REFERENCE_TINY_START - IMMEDIATE_TINY_START;
const BACK_REFERENCE_MAX: u8 = GLOBAL_INDEX_TINY_START - BACK_REFERENCE_TINY_START;
const GLOBAL_INDEX_MAX: u8 = RESERVED - GLOBAL_INDEX_TINY_START;

pub fn read_object(
    bytes: &mut ByteStream,
    length: usize,
    keys_table: &mut KeysTables,
) -> Result<Value, String> {
    if length == 0 {
        return Ok(Value::Object(Map::new()));
    }
    let mut map = Map::with_capacity(length);
    for _ in 0..length {
        let key = read_key(bytes, keys_table)?;
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
        write_key(key.clone(), bytes, keys_table)?;
        write_value(value, bytes, keys_table)?;
    }
    Ok(())
}

fn read_key(bytes: &mut ByteStream, keys_table: &mut KeysTables) -> Result<String, String> {
    let first_byte = bytes.read_u8()?;
    if first_byte < IMMEDIATE_TINY_START {
        let value = read_vu16(bytes)? as usize;
        match first_byte {
            0 => {
                let key = read_immediate_key(bytes, value, keys_table)?;
                return Ok(key);
            }
            1 => {
                let key = keys_table.lookup_local_index(value)?;
                return Ok(key.to_string());
            }
            2 => {
                let key = keys_table.lookup_global_index(value)?;
                return Ok(key.to_string());
            }
            _ => return Err(format!("Invalid key index byte: {:02X}", first_byte)),
        }
    }
    if first_byte < BACK_REFERENCE_TINY_START {
        let size = read_tiny_u8(first_byte, IMMEDIATE_TINY_START)?;
        let key = read_immediate_key(bytes, size as usize, keys_table)?;
        return Ok(key);
    }
    if first_byte < GLOBAL_INDEX_TINY_START {
        let key_index = read_tiny_u8(first_byte, BACK_REFERENCE_TINY_START)?;
        let key = keys_table.lookup_local_index(key_index as usize)?;
        return Ok(key.to_string());
    }
    if first_byte < RESERVED {
        let key_index = read_tiny_u8(first_byte, GLOBAL_INDEX_TINY_START)?;
        let key = keys_table.lookup_global_index(key_index as usize)?;
        return Ok(key.to_string());
    }
    return Err(format!("Invalid key index byte: {:02X}", first_byte));
}

fn write_key(
    key: String,
    bytes: &mut ByteStream,
    keys_table: &mut KeysTables,
) -> Result<(), String> {
    if let Some(global_index) = keys_table.find_global_index(&key) {
        write_type_and_value(
            bytes,
            global_index,
            2,
            GLOBAL_INDEX_TINY_START,
            GLOBAL_INDEX_MAX,
        )?;
        return Ok(());
    }
    if let Some(local_index) = keys_table.find_local_index(&key) {
        write_type_and_value(
            bytes,
            local_index,
            1,
            BACK_REFERENCE_TINY_START,
            BACK_REFERENCE_MAX,
        )?;
        return Ok(());
    }
    write_type_and_value(bytes, key.len(), 0, IMMEDIATE_TINY_START, IMMEDIATE_MAX)?;
    write_immediate_key(key, bytes, keys_table)?;
    Ok(())
}

fn write_type_and_value(
    bytes: &mut ByteStream,
    value: usize,
    vu8_offset: u8,
    tiny_start: u8,
    tiny_max: u8,
) -> Result<(), String> {
    if value < tiny_max as usize {
        return write_tiny_u8(value as u8, tiny_start, bytes);
    }
    bytes.write_u8(vu8_offset as u8)?;
    write_vu16(value as u16, bytes)?;
    Ok(())
}

fn read_tiny_u8(value: u8, start: u8) -> Result<u8, String> {
    Ok(value - start)
}

fn write_tiny_u8(value: u8, start: u8, bytes: &mut ByteStream) -> Result<(), String> {
    bytes.write_u8(value + start)
}

fn read_immediate_key(
    bytes: &mut ByteStream,
    length: usize,
    keys_table: &mut KeysTables,
) -> Result<String, String> {
    let key = bytes.read_string(length)?;
    keys_table.on_immediate_key(key.clone())?;
    Ok(key)
}

fn write_immediate_key(
    key: String,
    bytes: &mut ByteStream,
    keys_table: &mut KeysTables,
) -> Result<(), String> {
    bytes.write_string(&key)?;
    keys_table.on_immediate_key(key)?;
    Ok(())
}

fn read_vu16(bytes: &mut ByteStream) -> Result<u16, String> {
    let b0 = bytes.read_u8()?;
    let has_more = b0 & 0x80 != 0;
    if !has_more {
        return Ok(b0 as u16 & 0x7F);
    }
    let b1 = bytes.read_u8()? as u16;
    let has_more = b1 & 0x80 != 0;
    if !has_more {
        return Ok(b0 as u16 & 0x7F | (b1 & 0x7F) << 7);
    }
    let b2 = bytes.read_u8()? as u16;
    if b2 > 0x03 {
        return Err(format!(
            "Invalid key index bytes: {:02X} {:02X} {:02X}",
            b0, b1, b2
        ));
    }
    Ok((b0 as u16 & 0x7F | (b1 & 0x7F) << 7 | (b2 & 0x03) << 14) as u16)
}

fn write_vu16(key_index: u16, bytes: &mut ByteStream) -> Result<(), String> {
    let mut b0 = (key_index & 0x7F) as u8;
    if key_index < 0x80 {
        bytes.write_u8(b0)?;
        return Ok(());
    }
    b0 |= 0x80;
    let mut b1 = ((key_index >> 7) & 0x7F) as u8;
    if key_index < 0x4000 {
        bytes.write(&[b0, b1])?;
        return Ok(());
    }
    b1 |= 0x80;
    let b2 = ((key_index >> 14) & 0x03) as u8;
    bytes.write(&[b0, b1, b2])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn convert_key_index_twice(key_index: u16, expected_bytes_count: usize) -> Result<(), String> {
        let mut bytes = ByteStream::new();
        write_vu16(key_index, &mut bytes).unwrap();
        assert_eq!(expected_bytes_count, bytes.len());
        bytes.seek(0)?;
        let result = read_vu16(&mut bytes).unwrap();
        assert_eq!(key_index, result);
        Ok(())
    }

    #[test]
    fn test_key_index() {
        convert_key_index_twice(0, 1).unwrap();
        convert_key_index_twice(0x7F, 1).unwrap();
        convert_key_index_twice(0x80, 2).unwrap();
        convert_key_index_twice(0x3FFF, 2).unwrap();
        convert_key_index_twice(0x4000, 3).unwrap();
        convert_key_index_twice(0xFFFF, 3).unwrap();
    }
}
