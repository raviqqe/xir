//! Blah-blah intermediate representation.

#![feature(allocator_api)]

extern crate alloc;

mod argument;
mod block;
mod context;
mod operation;
mod span;
mod r#type;

pub use block::*;
pub use context::*;
pub use operation::*;
pub use r#type::*;
