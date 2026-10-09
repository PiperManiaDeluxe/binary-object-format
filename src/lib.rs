#![allow(dead_code)]

mod macros;

pub mod read;
pub mod write;

mod float;
pub use float::*;

mod integer;
pub use integer::*;

mod value;
pub use value::*;

pub(crate) mod spec;

extern crate alloc;

// Used for macro generated code.
pub mod __private {
    pub use alloc::vec;
}
