#![allow(dead_code)]

pub struct DataTypes;

impl DataTypes {
    pub const NULL: u8 = 0;
    pub const FALSE: u8 = 1;
    pub const TRUE: u8 = 2;
    pub const INT8: u8 = 3;
    pub const UINT8: u8 = 4;
    pub const INT16: u8 = 5;
    pub const UINT16: u8 = 6;
    pub const INT32: u8 = 7;
    pub const UINT32: u8 = 8;
    pub const INT64: u8 = 9;
    pub const UINT64: u8 = 0xA;
    pub const FLOAT: u8 = 0xB;
    pub const DOUBLE: u8 = 0xC;
    pub const STRING8: u8 = 0xD;
    pub const STRING16: u8 = 0xE;
    pub const STRING32: u8 = 0xF;
    pub const OBJECT8: u8 = 0x10;
    pub const OBJECT16: u8 = 0x11;
    pub const OBJECT32: u8 = 0x12;
    pub const ARRAY8: u8 = 0x13;
    pub const ARRAY16: u8 = 0x14;
    pub const ARRAY32: u8 = 0x15;
    pub const TINY_STRING: u8 = 0x16;
    pub const TINY_OBJECT: u8 = 0x6E;
    pub const TINY_ARRAY: u8 = 0x9E;
    pub const TINY_INT: u8 = 0xBE;
    pub const RESERVED: u8 = 0xFE;

    pub const TINY_INT_BIAS: i8 = -32;
    pub const TINY_INT_MIN: i8 = DataTypes::TINY_INT_BIAS;
    pub const TINY_INT_MAX: i8 =
        DataTypes::RESERVED as i8 - DataTypes::TINY_INT as i8 + DataTypes::TINY_INT_BIAS;
}
