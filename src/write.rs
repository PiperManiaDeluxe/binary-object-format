use std::{
    collections::BTreeMap,
    io::{self, Write},
};

use thiserror::Error;

use crate::{float::Float, integer::*, spec::*, value::*};

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("IO Error while writing to writer: {0}")]
    Io(#[from] io::Error),
}

pub fn write_value<W>(writer: &mut W, val: &Value) -> Result<(), WriteError>
where
    W: ?Sized + Write,
{
    match val {
        Value::Null => write_null(writer),
        Value::Bool(val) => write_bool(writer, val),
        Value::Integer(val) => write_integer(writer, val),
        Value::Float(val) => write_float(writer, val),
        Value::String(val) => write_string(writer, &val),
        Value::Array(val) => write_array(writer, val),
        Value::Object(val) => write_object(writer, val),
    }
}

fn write_null<W>(writer: &mut W) -> Result<(), WriteError>
where
    W: ?Sized + Write,
{
    writer.write_all(&[ID_NULL])?;

    Ok(())
}

fn write_bool<W>(writer: &mut W, val: &bool) -> Result<(), WriteError>
where
    W: ?Sized + Write,
{
    match val {
        true => writer.write_all(&[ID_BOOL_TRUE])?,
        false => writer.write_all(&[ID_BOOL_FALSE])?,
    };

    Ok(())
}

fn write_integer<W>(writer: &mut W, val: &Integer) -> Result<(), WriteError>
where
    W: ?Sized + Write,
{
    writer.write_all(&[ID_INTEGER, val.binary_identifier()])?;

    match val {
        Integer::U8(val) => writer.write_all(&val.to_le_bytes())?,
        Integer::U16(val) => writer.write_all(&val.to_le_bytes())?,
        Integer::U32(val) => writer.write_all(&val.to_le_bytes())?,
        Integer::U64(val) => writer.write_all(&val.to_le_bytes())?,
        Integer::U128(val) => writer.write_all(&val.to_le_bytes())?,
        Integer::I8(val) => writer.write_all(&val.to_le_bytes())?,
        Integer::I16(val) => writer.write_all(&val.to_le_bytes())?,
        Integer::I32(val) => writer.write_all(&val.to_le_bytes())?,
        Integer::I64(val) => writer.write_all(&val.to_le_bytes())?,
        Integer::I128(val) => writer.write_all(&val.to_le_bytes())?,
    }

    Ok(())
}

fn write_float<W>(writer: &mut W, val: &Float) -> Result<(), WriteError>
where
    W: ?Sized + Write,
{
    writer.write_all(&[ID_FLOAT, val.binary_identifier()])?;

    match val {
        Float::F32(val) => writer.write_all(&val.to_le_bytes())?,
        Float::F64(val) => writer.write_all(&val.to_le_bytes())?,
    }

    Ok(())
}

fn write_string<W>(writer: &mut W, val: &str) -> Result<(), WriteError>
where
    W: ?Sized + Write,
{
    writer.write_all(&[ID_STRING])?;

    let string_bytes = val.as_bytes();
    let length = string_bytes.len();

    writer.write_all(&(length as u64).to_le_bytes())?;
    writer.write_all(string_bytes)?;

    Ok(())
}

fn write_array<W>(writer: &mut W, val: &Vec<Value>) -> Result<(), WriteError>
where
    W: ?Sized + Write,
{
    writer.write_all(&[ID_ARRAY])?;

    let mut array_bytes: Vec<u8> = vec![];
    for value in val {
        write_value(&mut array_bytes, value)?;
    }

    let length = array_bytes.len();

    writer.write_all(&(length as u64).to_le_bytes())?;
    writer.write_all(&array_bytes)?;

    Ok(())
}

fn write_object<W>(writer: &mut W, val: &BTreeMap<KeyValue, Value>) -> Result<(), WriteError>
where
    W: ?Sized + Write,
{
    writer.write_all(&[ID_OBJECT])?;

    let key_value_pairs = val.len();
    let pairs_bytes = (key_value_pairs as u64).to_le_bytes();
    writer.write_all(&pairs_bytes)?;

    for (key, value) in val.iter() {
        let key = match key {
            KeyValue::Null => Value::Null,
            KeyValue::Bool(bool) => Value::Bool(*bool),
            KeyValue::Integer(integer) => Value::Integer(*integer),
            KeyValue::String(string) => Value::String(string.clone()),
        };

        write_value(writer, &key)?;
        write_value(writer, value)?;
    }

    Ok(())
}
