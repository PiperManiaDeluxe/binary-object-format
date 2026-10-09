use crate::spec::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Integer {
    // Unsigned integer
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    // Signed integer
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
}

pub fn to_integer<T>(value: T) -> Integer
where
    T: Into<Integer>,
{
    value.into()
}

impl From<u8> for Integer {
    fn from(value: u8) -> Self {
        Integer::U8(value)
    }
}

impl From<u16> for Integer {
    fn from(value: u16) -> Self {
        Integer::U16(value)
    }
}

impl From<u32> for Integer {
    fn from(value: u32) -> Self {
        Integer::U32(value)
    }
}

impl From<u64> for Integer {
    fn from(value: u64) -> Self {
        Integer::U64(value)
    }
}

impl From<u128> for Integer {
    fn from(value: u128) -> Self {
        Integer::U128(value)
    }
}

impl From<i8> for Integer {
    fn from(value: i8) -> Self {
        Integer::I8(value)
    }
}

impl From<i16> for Integer {
    fn from(value: i16) -> Self {
        Integer::I16(value)
    }
}

impl From<i32> for Integer {
    fn from(value: i32) -> Self {
        Integer::I32(value)
    }
}

impl From<i64> for Integer {
    fn from(value: i64) -> Self {
        Integer::I64(value)
    }
}

impl From<i128> for Integer {
    fn from(value: i128) -> Self {
        Integer::I128(value)
    }
}

impl Integer {
    pub const fn binary_identifier(&self) -> u8 {
        match self {
            Self::U8(_) => ID_NUM_TYPE_U8,
            Self::U16(_) => ID_NUM_TYPE_U16,
            Self::U32(_) => ID_NUM_TYPE_U32,
            Self::U64(_) => ID_NUM_TYPE_U64,
            Self::U128(_) => ID_NUM_TYPE_U128,
            Self::I8(_) => ID_NUM_TYPE_I8,
            Self::I16(_) => ID_NUM_TYPE_I16,
            Self::I32(_) => ID_NUM_TYPE_I32,
            Self::I64(_) => ID_NUM_TYPE_I64,
            Self::I128(_) => ID_NUM_TYPE_I128,
        }
    }
}
