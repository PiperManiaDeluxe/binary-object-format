use std::collections::BTreeMap;

use anyhow::Result;
use binary_object_format::{bof, read::read_value, write::write_value};

fn main() -> Result<()> {
    let null_value = bof!(null);
    let bool_value = bof!(true);
    let integer_value = bof!(42i8);
    let float_value = bof!(2.22f32);
    let string_value = bof!("I am a string!");
    let array_value = bof!([222, "I am in a array!", null]);
    let object_value = bof!({"true": true, false: "false"});

    let values = vec![
        null_value,
        bool_value,
        integer_value,
        float_value,
        string_value,
        array_value,
        object_value,
    ];

    dbg!(&values);

    // WRITE
    let mut bytes: Vec<u8> = vec![];
    for value in &values {
        write_value(&mut bytes, value)?;
    }

    // READ
    let mut bytes = bytes.as_slice();
    for expected_value in values {
        assert_eq!(Some(expected_value), read_value(&mut bytes)?)
    }

    Ok(())
}
