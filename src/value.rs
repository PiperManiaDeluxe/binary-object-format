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

pub fn to_value<T>(value: T) -> Value
where
    T: Into<Value>,
{
    value.into()
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Integer(Integer::I64(value))
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(Float::F64(value))
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

/// Represents valid BOF values that can be used as [`Value::Object`] keys.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyValue {
    Null,
    Bool(bool),
    Integer(Integer),
    String(String),
}

impl From<bool> for KeyValue {
    fn from(value: bool) -> Self {
        KeyValue::Bool(value)
    }
}

impl From<i64> for KeyValue {
    fn from(value: i64) -> Self {
        KeyValue::Integer(Integer::I64(value))
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
