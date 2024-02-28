use crate::byte_stream::ByteStream;

pub enum DataTypeCategory {
    Object,
    Array,
    String,
    Integer,
    Float,
    Bool,
    Null,
}

pub struct DataType {
    pub char: u8,
    pub category: DataTypeCategory,
    pub size_field_bytes: u8,
}

impl DataType {
    pub fn from_char(c: u8) -> Result<DataType, String> {
        match c {
            b'o' => Ok(DataTypes::EMPTY_OBJECT),
            b'O' => Ok(DataTypes::SMALL_OBJECT),
            b'p' => Ok(DataTypes::BIG_OBJECT),
            b'P' => Ok(DataTypes::LONG_OBJECT),
            b'a' => Ok(DataTypes::EMPTY_ARRAY),
            b'A' => Ok(DataTypes::SMALL_ARRAY),
            b'c' => Ok(DataTypes::BIG_ARRAY),
            b'C' => Ok(DataTypes::LONG_ARRAY),
            b's' => Ok(DataTypes::EMPTY_STRING),
            b'S' => Ok(DataTypes::SMALL_STRING),
            b't' => Ok(DataTypes::BIG_STRING),
            b'T' => Ok(DataTypes::LONG_STRING),
            b'0' => Ok(DataTypes::INTEGER_0),
            b'i' => Ok(DataTypes::INTEGER_I8),
            b'I' => Ok(DataTypes::INTEGER_U8),
            b'j' => Ok(DataTypes::INTEGER_I16),
            b'J' => Ok(DataTypes::INTEGER_U16),
            b'k' => Ok(DataTypes::INTEGER_I32),
            b'K' => Ok(DataTypes::INTEGER_U32),
            b'l' => Ok(DataTypes::INTEGER_I64),
            b'L' => Ok(DataTypes::INTEGER_U64),
            b'f' => Ok(DataTypes::FLOAT_0),
            b'F' => Ok(DataTypes::FLOAT_F32),
            b'd' => Ok(DataTypes::FLOAT_F64),
            b'b' => Ok(DataTypes::BOOL_FALSE),
            b'B' => Ok(DataTypes::BOOL_TRUE),
            b'z' => Ok(DataTypes::NULL),
            _ => Err(format!("Invalid data type char {}", c)),
        }
    }

    pub fn read_type_size(&self, bytes: &mut ByteStream) -> Result<usize, String> {
        let length: usize;
        if self.size_field_bytes == 0 {
            length = 0;
        } else if self.size_field_bytes == 1 {
            length = bytes.read_u8()? as usize;
        } else if self.size_field_bytes == 2 {
            length = bytes.read_u16()? as usize;
        } else if self.size_field_bytes == 4 {
            length = bytes.read_u32()? as usize;
        } else {
            return Err(format!(
                "Invalid size field bytes: {}",
                self.size_field_bytes
            ));
        }
        return Ok(length);
    }

    pub fn write_char_and_size_class(
        size: usize,
        data_type_chars: &[u8; 4],
        bytes: &mut ByteStream,
    ) -> Result<(), String> {
        if size == 0 {
            bytes.write_u8(data_type_chars[0])
        } else if size <= 0xFF {
            bytes.write_u8(data_type_chars[1])?;
            bytes.write_u8(size as u8)
        } else if size <= 0xFFFF {
            bytes.write_u8(data_type_chars[2])?;
            bytes.write_u16(size as u16)
        } else if size <= 0xFFFFFFFF {
            bytes.write_u8(data_type_chars[3])?;
            bytes.write_u32(size as u32)
        } else {
            Err(format!("Size too big: {size}"))
        }
    }
}

pub struct DataTypes;

impl DataTypes {
    pub const EMPTY_OBJECT: DataType = DataType {
        char: b'o',
        category: DataTypeCategory::Object,
        size_field_bytes: 0,
    };
    pub const SMALL_OBJECT: DataType = DataType {
        char: b'O',
        category: DataTypeCategory::Object,
        size_field_bytes: 1,
    };
    pub const BIG_OBJECT: DataType = DataType {
        char: b'p',
        category: DataTypeCategory::Object,
        size_field_bytes: 2,
    };
    pub const LONG_OBJECT: DataType = DataType {
        char: b'P',
        category: DataTypeCategory::Object,
        size_field_bytes: 4,
    };
    pub const EMPTY_ARRAY: DataType = DataType {
        char: b'a',
        category: DataTypeCategory::Array,
        size_field_bytes: 0,
    };
    pub const SMALL_ARRAY: DataType = DataType {
        char: b'A',
        category: DataTypeCategory::Array,
        size_field_bytes: 1,
    };
    pub const BIG_ARRAY: DataType = DataType {
        char: b'c',
        category: DataTypeCategory::Array,
        size_field_bytes: 2,
    };
    pub const LONG_ARRAY: DataType = DataType {
        char: b'C',
        category: DataTypeCategory::Array,
        size_field_bytes: 4,
    };
    pub const EMPTY_STRING: DataType = DataType {
        char: b's',
        category: DataTypeCategory::String,
        size_field_bytes: 0,
    };
    pub const SMALL_STRING: DataType = DataType {
        char: b'S',
        category: DataTypeCategory::String,
        size_field_bytes: 1,
    };
    pub const BIG_STRING: DataType = DataType {
        char: b't',
        category: DataTypeCategory::String,
        size_field_bytes: 2,
    };
    pub const LONG_STRING: DataType = DataType {
        char: b'T',
        category: DataTypeCategory::String,
        size_field_bytes: 4,
    };
    pub const INTEGER_0: DataType = DataType {
        char: b'0',
        category: DataTypeCategory::Integer,
        size_field_bytes: 0,
    };
    pub const INTEGER_I8: DataType = DataType {
        char: b'i',
        category: DataTypeCategory::Integer,
        size_field_bytes: 0,
    };
    pub const INTEGER_U8: DataType = DataType {
        char: b'I',
        category: DataTypeCategory::Integer,
        size_field_bytes: 0,
    };
    pub const INTEGER_I16: DataType = DataType {
        char: b'j',
        category: DataTypeCategory::Integer,
        size_field_bytes: 0,
    };
    pub const INTEGER_U16: DataType = DataType {
        char: b'J',
        category: DataTypeCategory::Integer,
        size_field_bytes: 0,
    };
    pub const INTEGER_I32: DataType = DataType {
        char: b'k',
        category: DataTypeCategory::Integer,
        size_field_bytes: 0,
    };
    pub const INTEGER_U32: DataType = DataType {
        char: b'K',
        category: DataTypeCategory::Integer,
        size_field_bytes: 0,
    };
    pub const INTEGER_I64: DataType = DataType {
        char: b'l',
        category: DataTypeCategory::Integer,
        size_field_bytes: 0,
    };
    pub const INTEGER_U64: DataType = DataType {
        char: b'L',
        category: DataTypeCategory::Integer,
        size_field_bytes: 0,
    };
    pub const FLOAT_0: DataType = DataType {
        char: b'f',
        category: DataTypeCategory::Float,
        size_field_bytes: 0,
    };
    pub const FLOAT_F32: DataType = DataType {
        char: b'F',
        category: DataTypeCategory::Float,
        size_field_bytes: 0,
    };
    pub const FLOAT_F64: DataType = DataType {
        char: b'd',
        category: DataTypeCategory::Float,
        size_field_bytes: 0,
    };
    pub const BOOL_FALSE: DataType = DataType {
        char: b'b',
        category: DataTypeCategory::Bool,
        size_field_bytes: 0,
    };
    pub const BOOL_TRUE: DataType = DataType {
        char: b'B',
        category: DataTypeCategory::Bool,
        size_field_bytes: 0,
    };
    pub const NULL: DataType = DataType {
        char: b'z',
        category: DataTypeCategory::Null,
        size_field_bytes: 0,
    };
}
