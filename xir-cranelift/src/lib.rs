//! Cranelift intermediate representation for XIR.

#![feature(allocator_api)]

mod operation;
mod r#type;

pub use operation::*;
pub use r#type::*;
