use crate::spec::{ID_NUM_TYPE_F32, ID_NUM_TYPE_F64};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Float {
    F32(f32),
    F64(f64),
}

impl Float {
    pub const fn binary_identifier(&self) -> u8 {
        match self {
            Self::F32(_) => ID_NUM_TYPE_F32,
            Self::F64(_) => ID_NUM_TYPE_F64,
        }
    }
}
