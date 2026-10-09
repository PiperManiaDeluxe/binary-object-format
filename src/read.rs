use std::{
    collections::BTreeMap,
    io::{self, Read},
};

use thiserror::Error;

use crate::{float::Float, integer::*, spec::*, value::*};

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("IO Error while reading from reader: {0}")]
    Io(#[from] io::Error),
    #[error("String from UTF-8 conversion failed: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[error("Read unknown type identifier: {0}")]
    UnknownID(u8),
    #[error("Read unknown number type identifier: {0}")]
    UnknownNumID(u8),

    #[error("Could not read object: a key is missing.")]
    MissingObjectKey,
    #[error("Could not read object: a value is missing.")]
    MissingObjectValue,

    #[error("The value type `{0}` is not usable as an object key.")]
    ValueTypeNotUsableAsKey(&'static str),
}

/// Reads one [`Value`] from a reader. Returns `None` when there is nothing left to read.
pub fn read_value<R>(reader: &mut R) -> Result<Option<Value>, ReadError>
where
    R: ?Sized + Read,
{
    let mut id_bytes = [0; 1];
    let bytes_read = reader.read(&mut id_bytes)?;

    if bytes_read == 0 {
        return Ok(None);
    }

    let id = id_bytes[0];

    match id {
        ID_NULL => Ok(Some(Value::Null)),
        ID_BOOL_TRUE => Ok(Some(Value::Bool(true))),
        ID_BOOL_FALSE => Ok(Some(Value::Bool(false))),
        ID_INTEGER => Ok(Some(Value::Integer(read_integer(reader)?))),
        ID_FLOAT => Ok(Some(Value::Float(read_float(reader)?))),
        ID_STRING => Ok(Some(Value::String(read_string(reader)?))),
        ID_ARRAY => Ok(Some(Value::Array(read_array(reader)?))),
        ID_OBJECT => Ok(Some(Value::Object(read_object(reader)?))),
        id_unknown => Err(ReadError::UnknownID(id_unknown)),
    }
}

fn read_integer<R>(reader: &mut R) -> Result<Integer, ReadError>
where
    R: ?Sized + Read,
{
    let mut num_id_bytes = [0; 1];
    reader.read_exact(&mut num_id_bytes)?;
    let num_id = num_id_bytes[0];

    match num_id {
        ID_NUM_TYPE_U8 => {
            let mut num_bytes = [0; 1];
            reader.read_exact(&mut num_bytes)?;
            Ok(Integer::U8(u8::from_le_bytes(num_bytes)))
        }
        ID_NUM_TYPE_U16 => {
            let mut num_bytes = [0; 2];
            reader.read_exact(&mut num_bytes)?;
            Ok(Integer::U16(u16::from_le_bytes(num_bytes)))
        }
        ID_NUM_TYPE_U32 => {
            let mut num_bytes = [0; 4];
            reader.read_exact(&mut num_bytes)?;
            Ok(Integer::U32(u32::from_le_bytes(num_bytes)))
        }
        ID_NUM_TYPE_U64 => {
            let mut num_bytes = [0; 8];
            reader.read_exact(&mut num_bytes)?;
            Ok(Integer::U64(u64::from_le_bytes(num_bytes)))
        }
        ID_NUM_TYPE_U128 => {
            let mut num_bytes = [0; 16];
            reader.read_exact(&mut num_bytes)?;
            Ok(Integer::U128(u128::from_le_bytes(num_bytes)))
        }
        ID_NUM_TYPE_I8 => {
            let mut num_bytes = [0; 1];
            reader.read_exact(&mut num_bytes)?;
            Ok(Integer::I8(i8::from_le_bytes(num_bytes)))
        }
        ID_NUM_TYPE_I16 => {
            let mut num_bytes = [0; 2];
            reader.read_exact(&mut num_bytes)?;
            Ok(Integer::I16(i16::from_le_bytes(num_bytes)))
        }
        ID_NUM_TYPE_I32 => {
            let mut num_bytes = [0; 4];
            reader.read_exact(&mut num_bytes)?;
            Ok(Integer::I32(i32::from_le_bytes(num_bytes)))
        }
        ID_NUM_TYPE_I64 => {
            let mut num_bytes = [0; 8];
            reader.read_exact(&mut num_bytes)?;
            Ok(Integer::I64(i64::from_le_bytes(num_bytes)))
        }
        ID_NUM_TYPE_I128 => {
            let mut num_bytes = [0; 16];
            reader.read_exact(&mut num_bytes)?;
            Ok(Integer::I128(i128::from_le_bytes(num_bytes)))
        }
        id_unknown => Err(ReadError::UnknownNumID(id_unknown)),
    }
}

fn read_float<R>(reader: &mut R) -> Result<Float, ReadError>
where
    R: ?Sized + Read,
{
    let mut num_id_bytes = [0; 1];
    reader.read_exact(&mut num_id_bytes)?;
    let num_id = num_id_bytes[0];

    match num_id {
        ID_NUM_TYPE_F32 => {
            let mut num_bytes = [0; 4];
            reader.read_exact(&mut num_bytes)?;
            Ok(Float::F32(f32::from_le_bytes(num_bytes)))
        }
        ID_NUM_TYPE_F64 => {
            let mut num_bytes = [0; 8];
            reader.read_exact(&mut num_bytes)?;
            Ok(Float::F64(f64::from_le_bytes(num_bytes)))
        }
        id_unknown => Err(ReadError::UnknownNumID(id_unknown)),
    }
}

fn read_string<R>(reader: &mut R) -> Result<String, ReadError>
where
    R: ?Sized + Read,
{
    let mut length = [0; 8];
    reader.read_exact(&mut length)?;
    let length = u64::from_le_bytes(length);

    let mut bytes = vec![0; length as usize];
    reader.read_exact(&mut bytes)?;
    let string = String::from_utf8(bytes)?;

    Ok(string)
}

fn read_array<R>(reader: &mut R) -> Result<Vec<Value>, ReadError>
where
    R: ?Sized + Read,
{
    let mut array_length_bytes = [0; 8];
    reader.read_exact(&mut array_length_bytes)?;
    let array_length = u64::from_le_bytes(array_length_bytes);

    let mut array_reader = (&mut *reader).take(array_length);
    let mut result = Vec::new();

    // We have to cast array_reader as `dyn` to avoid compile time type recursion.
    while let Some(value) = read_value(&mut array_reader as &mut dyn Read)? {
        result.push(value);
    }

    Ok(result)
}

fn read_object<R>(reader: &mut R) -> Result<BTreeMap<KeyValue, Value>, ReadError>
where
    R: ?Sized + Read,
{
    let mut pairs_bytes = [0; 8];
    reader.read_exact(&mut pairs_bytes)?;
    let key_value_pairs = u64::from_le_bytes(pairs_bytes);

    let mut map = BTreeMap::new();

    for _ in 0..key_value_pairs {
        let Some(key) = read_value(reader)? else {
            return Err(ReadError::MissingObjectKey);
        };

        let key = match key {
            Value::Null => KeyValue::Null,
            Value::Bool(bool) => KeyValue::Bool(bool),
            Value::Integer(integer) => KeyValue::Integer(integer),
            Value::String(string) => KeyValue::String(string),
            Value::Float(_) => return Err(ReadError::ValueTypeNotUsableAsKey("Value::Float")),
            Value::Array(_) => return Err(ReadError::ValueTypeNotUsableAsKey("Value::Array")),
            Value::Object(_) => return Err(ReadError::ValueTypeNotUsableAsKey("Value::Object")),
        };

        let Some(value) = read_value(reader)? else {
            return Err(ReadError::MissingObjectValue);
        };

        map.insert(key, value);
    }

    Ok(map)
}
