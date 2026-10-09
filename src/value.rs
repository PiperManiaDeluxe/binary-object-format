use std::collections::BTreeMap;

use crate::{float::Float, integer::Integer};

/// Represents any valid BOF value.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Null,
    Bool(bool),
    Integer(Integer),
    Float(Float),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<KeyValue, Value>),
}

// #region From<Type>

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<Integer> for Value {
    fn from(value: Integer) -> Self {
        Value::Integer(value)
    }
}

impl From<Float> for Value {
    fn from(value: Float) -> Self {
        Value::Float(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::Integer(value.into())
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::Integer(value.into())
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::Integer(value.into())
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::Integer(value.into())
    }
}

impl From<u128> for Value {
    fn from(value: u128) -> Self {
        Value::Integer(value.into())
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::Integer(value.into())
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::Integer(value.into())
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Integer(value.into())
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Integer(value.into())
    }
}

impl From<i128> for Value {
    fn from(value: i128) -> Self {
        Value::Integer(value.into())
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Float(value.into())
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value.into())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_owned())
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(values: Vec<T>) -> Self {
        Value::Array(values.into_iter().map(Into::into).collect())
    }
}

impl<T: Into<Value>> From<BTreeMap<KeyValue, T>> for Value {
    fn from(values: BTreeMap<KeyValue, T>) -> Self {
        Value::Object(
            values
                .into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
        )
    }
}

// #endregion

/// Represents valid BOF values that can be used as [`Value::Object`] keys.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyValue {
    Null,
    Bool(bool),
    Integer(Integer),
    String(String),
}

// #region From<Type>

impl From<bool> for KeyValue {
    fn from(value: bool) -> Self {
        KeyValue::Bool(value)
    }
}

impl From<u8> for KeyValue {
    fn from(value: u8) -> Self {
        KeyValue::Integer(value.into())
    }
}

impl From<u16> for KeyValue {
    fn from(value: u16) -> Self {
        KeyValue::Integer(value.into())
    }
}

impl From<u32> for KeyValue {
    fn from(value: u32) -> Self {
        KeyValue::Integer(value.into())
    }
}

impl From<u64> for KeyValue {
    fn from(value: u64) -> Self {
        KeyValue::Integer(value.into())
    }
}

impl From<u128> for KeyValue {
    fn from(value: u128) -> Self {
        KeyValue::Integer(value.into())
    }
}

impl From<i8> for KeyValue {
    fn from(value: i8) -> Self {
        KeyValue::Integer(value.into())
    }
}

impl From<i16> for KeyValue {
    fn from(value: i16) -> Self {
        KeyValue::Integer(value.into())
    }
}

impl From<i32> for KeyValue {
    fn from(value: i32) -> Self {
        KeyValue::Integer(value.into())
    }
}

impl From<i64> for KeyValue {
    fn from(value: i64) -> Self {
        KeyValue::Integer(value.into())
    }
}

impl From<i128> for KeyValue {
    fn from(value: i128) -> Self {
        KeyValue::Integer(value.into())
    }
}

impl From<String> for KeyValue {
    fn from(value: String) -> Self {
        KeyValue::String(value)
    }
}

impl From<&str> for KeyValue {
    fn from(value: &str) -> Self {
        KeyValue::String(value.to_owned())
    }
}

// #endregion
