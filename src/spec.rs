//! Constants related to BOF spec

// 0 is reserved for meta data+
pub const ID_NULL: u8 = 1;
pub const ID_BOOL_TRUE: u8 = 2;
pub const ID_BOOL_FALSE: u8 = 3;
pub const ID_INTEGER: u8 = 4;
pub const ID_FLOAT: u8 = 5;
pub const ID_STRING: u8 = 6;
pub const ID_ARRAY: u8 = 7;
pub const ID_OBJECT: u8 = 8;

pub const ID_NUM_TYPE_U8: u8 = 1;
pub const ID_NUM_TYPE_U16: u8 = 2;
pub const ID_NUM_TYPE_U32: u8 = 3;
pub const ID_NUM_TYPE_U64: u8 = 4;
pub const ID_NUM_TYPE_U128: u8 = 5;
pub const ID_NUM_TYPE_I8: u8 = 6;
pub const ID_NUM_TYPE_I16: u8 = 7;
pub const ID_NUM_TYPE_I32: u8 = 8;
pub const ID_NUM_TYPE_I64: u8 = 9;
pub const ID_NUM_TYPE_I128: u8 = 10;
pub const ID_NUM_TYPE_F32: u8 = 11;
pub const ID_NUM_TYPE_F64: u8 = 12;
