#![allow(dead_code)]

pub mod macros;

pub mod read;
pub(crate) mod spec;
pub mod write;

mod float;
pub use float::*;

mod integer;
pub use integer::*;

mod value;
pub use value::*;

extern crate alloc;

// Used for macro generated code.
pub mod __private {
    pub use alloc::vec;
}
