use serde_json::{Map, Value};

use crate::{
    byte_stream::{ByteReader, ByteWriter},
    keys_table::{DecodeKeysTables, EncodeKeysTables, MAX_KEY_LENGTH},
};

use super::value::{read_value, write_value};

const IMMEDIATE_TINY_START: u8 = 0x03;
const BACK_REFERENCE_TINY_START: u8 = 0x57;
const GLOBAL_INDEX_TINY_START: u8 = 0xAB;
const RESERVED: u8 = 0xFF;
const IMMEDIATE_MAX: u8 = BACK_REFERENCE_TINY_START - IMMEDIATE_TINY_START;
const BACK_REFERENCE_MAX: u8 = GLOBAL_INDEX_TINY_START - BACK_REFERENCE_TINY_START;
const GLOBAL_INDEX_MAX: u8 = RESERVED - GLOBAL_INDEX_TINY_START;

pub fn read_object(
    bytes: &mut ByteReader,
    length: usize,
    keys_table: &mut DecodeKeysTables,
) -> Result<Value, String> {
    if length == 0 {
        return Ok(Value::Object(Map::new()));
    }
    let mut map = Map::with_capacity(length);
    for _ in 0..length {
        let key = read_key(bytes, keys_table)?;
        let value = read_value(bytes, keys_table)?;
        map.insert(key, value);
    }
    return Ok(Value::Object(map));
}

pub fn write_object<'a, 'b: 'a, W: ByteWriter>(
    object: &'b Map<String, Value>,
    bytes: &mut W,
    keys_table: &mut EncodeKeysTables<'a>,
) -> Result<(), String> {
    for (key, value) in object {
        write_key(key, bytes, keys_table)?;
        write_value(value, bytes, keys_table)?;
    }
    Ok(())
}

fn read_key(bytes: &mut ByteReader, keys_table: &mut DecodeKeysTables) -> Result<String, String> {
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
                return Ok(key.clone());
            }
            2 => {
                let key = keys_table.lookup_global_index(value)?;
                return Ok(key.clone());
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
        return Ok(key.clone());
    }
    if first_byte < RESERVED {
        let key_index = read_tiny_u8(first_byte, GLOBAL_INDEX_TINY_START)?;
        let key = keys_table.lookup_global_index(key_index as usize)?;
        return Ok(key.clone());
    }
    return Err(format!("Invalid key index byte: {:02X}", first_byte));
}

fn write_key<'a, 'b: 'a, W: ByteWriter>(
    key: &'b String,
    bytes: &mut W,
    keys_table: &mut EncodeKeysTables<'a>,
) -> Result<(), String> {
    if key.len() > MAX_KEY_LENGTH {
        return Err(format!(
            "Key length {} exceeds MAX_KEY_LENGTH {}",
            key.len(),
            MAX_KEY_LENGTH
        ));
    }
    if let Some(global_index) = keys_table.find_global_index(&key) {
        write_type_and_value(
            bytes,
            global_index,
            2,
            GLOBAL_INDEX_TINY_START,
            GLOBAL_INDEX_MAX,
        );
        return Ok(());
    }
    if let Some(local_index) = keys_table.find_local_index(&key) {
        write_type_and_value(
            bytes,
            local_index,
            1,
            BACK_REFERENCE_TINY_START,
            BACK_REFERENCE_MAX,
        );
        return Ok(());
    }
    write_type_and_value(bytes, key.len(), 0, IMMEDIATE_TINY_START, IMMEDIATE_MAX);
    write_immediate_key(key, bytes, keys_table)?;
    Ok(())
}

fn write_type_and_value<W: ByteWriter>(
    bytes: &mut W,
    value: usize,
    vu8_offset: u8,
    tiny_start: u8,
    tiny_max: u8,
) {
    if value < tiny_max as usize {
        write_tiny_u8(value as u8, tiny_start, bytes);
        return;
    }
    bytes.write_u8(vu8_offset as u8);
    write_vu16(value as u16, bytes);
}

fn read_tiny_u8(value: u8, start: u8) -> Result<u8, String> {
    Ok(value - start)
}

fn write_tiny_u8<W: ByteWriter>(value: u8, start: u8, bytes: &mut W) {
    bytes.write_u8(value + start);
}

fn read_immediate_key(
    bytes: &mut ByteReader,
    length: usize,
    keys_table: &mut DecodeKeysTables,
) -> Result<String, String> {
    let key = bytes.read_string(length)?;
    keys_table.on_immediate_key(&key);
    Ok(key)
}

fn write_immediate_key<'a, 'b: 'a, W: ByteWriter>(
    key: &'b String,
    bytes: &mut W,
    keys_table: &mut EncodeKeysTables<'a>,
) -> Result<(), String> {
    bytes.write_string(&key);
    keys_table.on_immediate_key(key);
    Ok(())
}

fn read_vu16(bytes: &mut ByteReader) -> Result<u16, String> {
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

fn write_vu16<W: ByteWriter>(key_index: u16, bytes: &mut W) {
    let mut b0 = (key_index & 0x7F) as u8;
    if key_index < 0x80 {
        bytes.write_u8(b0);
        return;
    }
    b0 |= 0x80;
    let mut b1 = ((key_index >> 7) & 0x7F) as u8;
    if key_index < 0x4000 {
        bytes.write(&[b0, b1]);
        return;
    }
    b1 |= 0x80;
    let b2 = ((key_index >> 14) & 0x03) as u8;
    bytes.write(&[b0, b1, b2]);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn convert_key_index_twice(key_index: u16, expected_bytes_count: usize) -> Result<(), String> {
        let mut bytes = Vec::new();
        write_vu16(key_index, &mut bytes);
        assert_eq!(expected_bytes_count, bytes.len());
        let mut bytes = ByteReader::make(&bytes);
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
