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
