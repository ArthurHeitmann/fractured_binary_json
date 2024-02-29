use serde_json::{json, Number, Value};

use crate::{byte_stream::ByteStream, keys_table::KeysTables};

use super::{
    array::{read_array_from_data_type, write_array},
    data_type::{DataType, DataTypeCategory, DataTypes},
    object::{read_object_from_data_type, write_object},
    string::{read_string_from_data_type, write_string},
};

pub fn read_value(bytes: &mut ByteStream, keys_table: &KeysTables) -> Result<Value, String> {
    let data_type_char = bytes.read_u8()?;
    let data_type = DataType::from_char(data_type_char)?;
    match data_type.category {
        DataTypeCategory::Object => read_object_from_data_type(bytes, data_type, keys_table),
        DataTypeCategory::Array => read_array_from_data_type(bytes, data_type, keys_table),
        DataTypeCategory::String => read_string_from_data_type(bytes, data_type),
        DataTypeCategory::Integer => {
            let n: Number;
            if DataTypes::INTEGER_0.char == data_type.char {
                n = Number::from(0)
            } else if DataTypes::INTEGER_U8.char == data_type.char {
                n = Number::from(bytes.read_u8()?)
            } else if DataTypes::INTEGER_I8.char == data_type.char {
                n = Number::from(bytes.read_i8()?)
            } else if DataTypes::INTEGER_U16.char == data_type.char {
                n = Number::from(bytes.read_u16()?)
            } else if DataTypes::INTEGER_I16.char == data_type.char {
                n = Number::from(bytes.read_i16()?)
            } else if DataTypes::INTEGER_U32.char == data_type.char {
                n = Number::from(bytes.read_u32()?)
            } else if DataTypes::INTEGER_I32.char == data_type.char {
                n = Number::from(bytes.read_i32()?)
            } else if DataTypes::INTEGER_U64.char == data_type.char {
                n = Number::from(bytes.read_u64()?)
            } else if DataTypes::INTEGER_I64.char == data_type.char {
                n = Number::from(bytes.read_i64()?)
            } else {
                return Err(format!("Unknown integer data type: {}", data_type.char));
            }
            Ok(Value::Number(n))
        }
        DataTypeCategory::Float => {
            if DataTypes::FLOAT_0.char == data_type.char {
                Ok(json!(0.0))
            } else if DataTypes::FLOAT_F32.char == data_type.char {
                Ok(json!(bytes.read_f32()?))
            } else if DataTypes::FLOAT_F64.char == data_type.char {
                Ok(json!(bytes.read_f64()?))
            } else {
                Err(format!("Unknown float data type: {}", data_type.char))
            }
        }
        DataTypeCategory::Bool => Ok(Value::Bool(data_type.char == DataTypes::BOOL_TRUE.char)),
        DataTypeCategory::Null => Ok(Value::Null),
    }
}

pub fn write_value(
    value: &Value,
    bytes: &mut ByteStream,
    keys_table: &mut KeysTables,
) -> Result<(), String> {
    match value {
        Value::Null => bytes.write_u8(DataTypes::NULL.char),
        Value::Bool(b) => match b {
            true => bytes.write_u8(DataTypes::BOOL_TRUE.char),
            false => bytes.write_u8(DataTypes::BOOL_FALSE.char),
        },
        Value::Number(number) => {
            if let Some(n) = number.as_i64() {
                if n == 0 {
                    bytes.write_u8(DataTypes::INTEGER_0.char)
                } else if n >= 0 {
                    if n <= 0xFF {
                        bytes.write_u8(DataTypes::INTEGER_U8.char)?;
                        bytes.write_u8(n as u8)
                    } else if n <= 0xFFFF {
                        bytes.write_u8(DataTypes::INTEGER_U16.char)?;
                        bytes.write_u16(n as u16)
                    } else if n <= 0xFFFFFFFF {
                        bytes.write_u8(DataTypes::INTEGER_U32.char)?;
                        bytes.write_u32(n as u32)
                    } else {
                        bytes.write_u8(DataTypes::INTEGER_U64.char)?;
                        bytes.write_u64(n as u64)
                    }
                } else {
                    if n >= -0x80 {
                        bytes.write_u8(DataTypes::INTEGER_I8.char)?;
                        bytes.write_i8(n as i8)
                    } else if n >= -0x8000 {
                        bytes.write_u8(DataTypes::INTEGER_I16.char)?;
                        bytes.write_i16(n as i16)
                    } else if n >= -0x80000000 {
                        bytes.write_u8(DataTypes::INTEGER_I32.char)?;
                        bytes.write_i32(n as i32)
                    } else {
                        bytes.write_u8(DataTypes::INTEGER_I64.char)?;
                        bytes.write_i64(n)
                    }
                }
            } else if let Some(n) = number.as_u64() {
                bytes.write_u8(DataTypes::INTEGER_U64.char)?;
                bytes.write_u64(n)
            } else if let Some(n) = number.as_f64() {
                if n == 0.0 {
                    bytes.write_u8(DataTypes::FLOAT_0.char)
                } else if can_be_represented_as_f32(n) {
                    bytes.write_u8(DataTypes::FLOAT_F32.char)?;
                    bytes.write_f32(n as f32)
                } else {
                    bytes.write_u8(DataTypes::FLOAT_F64.char)?;
                    bytes.write_f64(n)
                }
            } else {
                Err("Number is not an integer or float".to_string())
            }
        }
        Value::String(string) => write_string(string, bytes),
        Value::Array(array) => write_array(array, bytes, keys_table),
        Value::Object(object) => write_object(object, bytes, keys_table),
    }
}

fn can_be_represented_as_f32(f: f64) -> bool {
    const SMALLEST_F32: f64 = 1.1754943508222875e-38;
    const LARGEST_INTEGER_F32: f64 = 16777216.0;
    let abs_f = f.abs();
    abs_f >= SMALLEST_F32 && abs_f <= LARGEST_INTEGER_F32
}
