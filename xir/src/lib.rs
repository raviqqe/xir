//! Blah-blah intermediate representation.

#![feature(allocator_api)]

extern crate alloc;

mod argument;
mod block;
mod context;
mod operation;
mod span;
mod r#type;
mod value;

pub use argument::*;
pub use block::*;
pub use context::*;
pub use operation::*;
pub use r#type::*;
pub use span::*;
pub use value::*;
