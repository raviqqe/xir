//! Blah-blah intermediate representation.

#![feature(allocator_api)]

extern crate alloc;

mod block;
mod context;
mod operation;
mod r#type;

pub use block::*;
pub use context::*;
pub use operation::*;
pub use r#type::*;
