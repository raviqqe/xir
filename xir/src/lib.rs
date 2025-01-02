//! Blah-blah intermediate representation.

#![feature(allocator_api)]

extern crate alloc;

mod attribute;
mod block;
mod block_argument;
mod context;
mod operation;
mod span;
mod symbol;
mod r#type;
mod value;

pub use attribute::*;
pub use block::*;
pub use block_argument::*;
pub use context::*;
pub use operation::*;
pub use r#type::*;
pub use span::*;
pub use symbol::*;
pub use value::*;
