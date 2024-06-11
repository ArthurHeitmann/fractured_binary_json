use serde_json::Value;

use crate::{
    byte_stream::{ByteReader, ByteWriter},
    keys_table::{DecodeKeysTables, EncodeKeysTables},
};

use super::{
    array::{read_array, write_array},
    data_type::DataTypes,
    object::{read_object, write_object},
    string::{read_string, write_string},
};

const READ_VALUE_FROM_TYPE: [fn(&mut ByteReader, &mut DecodeKeysTables) -> Result<Value, String>;
    22] = [
    |_, _| Ok(Value::Null),
    |_, _| Ok(Value::Bool(false)),
    |_, _| Ok(Value::Bool(true)),
    |bytes, _| Ok(Value::from(bytes.read_i8()?)),
    |bytes, _| Ok(Value::from(bytes.read_u8()?)),
    |bytes, _| Ok(Value::from(bytes.read_i16()?)),
    |bytes, _| Ok(Value::from(bytes.read_u16()?)),
    |bytes, _| Ok(Value::from(bytes.read_i32()?)),
    |bytes, _| Ok(Value::from(bytes.read_u32()?)),
    |bytes, _| Ok(Value::from(bytes.read_i64()?)),
    |bytes, _| Ok(Value::from(bytes.read_u64()?)),
    |bytes, _| Ok(Value::from(bytes.read_f32()?)),
    |bytes, _| Ok(Value::from(bytes.read_f64()?)),
    |bytes, _| {
        let length = bytes.read_u8()? as usize;
        read_string(bytes, length)
    },
    |bytes, _| {
        let length = bytes.read_u16()? as usize;
        read_string(bytes, length)
    },
    |bytes, _| {
        let length = bytes.read_u32()? as usize;
        read_string(bytes, length)
    },
    |bytes, keys_table| {
        let length = bytes.read_u8()? as usize;
        read_object(bytes, length, keys_table)
    },
    |bytes, keys_table| {
        let length = bytes.read_u16()? as usize;
        read_object(bytes, length, keys_table)
    },
    |bytes, keys_table| {
        let length = bytes.read_u32()? as usize;
        read_object(bytes, length, keys_table)
    },
    |bytes, keys_table| {
        let length = bytes.read_u8()? as usize;
        read_array(bytes, length, keys_table)
    },
    |bytes, keys_table| {
        let length = bytes.read_u16()? as usize;
        read_array(bytes, length, keys_table)
    },
    |bytes, keys_table| {
        let length = bytes.read_u32()? as usize;
        read_array(bytes, length, keys_table)
    },
];

pub fn read_value(
    bytes: &mut ByteReader,
    keys_table: &mut DecodeKeysTables,
) -> Result<Value, String> {
    let data_type_char = bytes.read_u8()?;
    if data_type_char < DataTypes::TINY_STRING {
        let index = data_type_char as usize;
        let value = READ_VALUE_FROM_TYPE[index](bytes, keys_table)?;
        return Ok(value);
    } else if data_type_char < DataTypes::TINY_OBJECT {
        let length = data_type_char - DataTypes::TINY_STRING;
        return read_string(bytes, length as usize);
    } else if data_type_char < DataTypes::TINY_ARRAY {
        let length = data_type_char - DataTypes::TINY_OBJECT;
        return read_object(bytes, length as usize, keys_table);
    } else if data_type_char < DataTypes::TINY_INT {
        let length = data_type_char - DataTypes::TINY_ARRAY;
        return read_array(bytes, length as usize, keys_table);
    } else if data_type_char < DataTypes::RESERVED {
        let value = (data_type_char - DataTypes::TINY_INT) as i8 + DataTypes::TINY_INT_BIAS;
        return Ok(Value::from(value));
    } else {
        return Err(format!("Reserved data type byte {}", data_type_char));
    }
}

pub fn write_value<'a, 'b: 'a, W: ByteWriter>(
    value: &'b Value,
    bytes: &mut W,
    keys_table: &mut EncodeKeysTables<'a>,
) -> Result<(), String> {
    match value {
        Value::Null => Ok(bytes.write_u8(DataTypes::NULL)),
        Value::Bool(b) => match b {
            false => Ok(bytes.write_u8(DataTypes::FALSE)),
            true => Ok(bytes.write_u8(DataTypes::TRUE)),
        },
        Value::Number(number) => {
            if let Some(n) = number.as_i64() {
                if n >= DataTypes::TINY_INT_MIN as i64 && n < DataTypes::TINY_INT_MAX as i64 {
                    let tiny_int =
                        (n - DataTypes::TINY_INT_BIAS as i64) as u8 + DataTypes::TINY_INT;
                    bytes.write_u8(tiny_int as u8);
                    Ok(())
                } else if n >= 0 {
                    if n <= 0xFF {
                        bytes.write_u8(DataTypes::UINT8);
                        bytes.write_u8(n as u8);
                        Ok(())
                    } else if n <= 0xFFFF {
                        bytes.write_u8(DataTypes::UINT16);
                        bytes.write_u16(n as u16);
                        Ok(())
                    } else if n <= 0xFFFFFFFF {
                        bytes.write_u8(DataTypes::UINT32);
                        bytes.write_u32(n as u32);
                        Ok(())
                    } else {
                        bytes.write_u8(DataTypes::UINT64);
                        bytes.write_u64(n as u64);
                        Ok(())
                    }
                } else {
                    if n >= -0x80 {
                        bytes.write_u8(DataTypes::INT8);
                        bytes.write_i8(n as i8);
                        Ok(())
                    } else if n >= -0x8000 {
                        bytes.write_u8(DataTypes::INT16);
                        bytes.write_i16(n as i16);
                        Ok(())
                    } else if n >= -0x80000000 {
                        bytes.write_u8(DataTypes::INT32);
                        bytes.write_i32(n as i32);
                        Ok(())
                    } else {
                        bytes.write_u8(DataTypes::INT64);
                        bytes.write_i64(n);
                        Ok(())
                    }
                }
            } else if let Some(n) = number.as_u64() {
                bytes.write_u8(DataTypes::UINT64);
                bytes.write_u64(n);
                Ok(())
            } else if let Some(n) = number.as_f64() {
                if can_be_represented_as_f32(n) {
                    bytes.write_u8(DataTypes::FLOAT);
                    bytes.write_f32(n as f32);
                    Ok(())
                } else {
                    bytes.write_u8(DataTypes::DOUBLE);
                    bytes.write_f64(n);
                    Ok(())
                }
            } else {
                Err("Number is not an integer or float".to_string())
            }
        }
        Value::String(string) => {
            write_var_length_data_type(
                string.len(),
                DataTypes::STRING8,
                DataTypes::TINY_STRING,
                DataTypes::TINY_OBJECT - DataTypes::TINY_STRING,
                bytes,
            )?;
            write_string(string, bytes);
            Ok(())
        }
        Value::Object(object) => {
            write_var_length_data_type(
                object.len(),
                DataTypes::OBJECT8,
                DataTypes::TINY_OBJECT,
                DataTypes::TINY_ARRAY - DataTypes::TINY_OBJECT,
                bytes,
            )?;
            write_object(object, bytes, keys_table)
        }
        Value::Array(array) => {
            write_var_length_data_type(
                array.len(),
                DataTypes::ARRAY8,
                DataTypes::TINY_ARRAY,
                DataTypes::TINY_INT - DataTypes::TINY_ARRAY,
                bytes,
            )?;
            write_array(array, bytes, keys_table)
        }
    }
}

fn can_be_represented_as_f32(f: f64) -> bool {
    if f.is_nan() {
        return false;
    }
    if f.is_infinite() {
        return true;
    }
    let f_with_f32_precision = f as f32 as f64;
    return f_with_f32_precision == f
}

fn write_var_length_data_type<W: ByteWriter>(
    length: usize,
    normal_offset: u8,
    tiny_offset: u8,
    tiny_max: u8,
    bytes: &mut W,
) -> Result<(), String> {
    if length < tiny_max as usize {
        bytes.write_u8(length as u8 + tiny_offset);
    } else {
        let additional_offset = if length <= 0xFF {
            0
        } else if length <= 0xFFFF {
            1
        } else if length <= 0xFFFFFFFF {
            2
        } else {
            return Err(format!("Value length {} is too long", length));
        };
        bytes.write_u8(normal_offset + additional_offset);
        match additional_offset {
            0 => bytes.write_u8(length as u8),
            1 => bytes.write_u16(length as u16),
            2 => bytes.write_u32(length as u32),
            _ => panic!("This should never happen"),
        }
    }
    Ok(())
}
