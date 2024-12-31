//! Blah-blah intermediate representation.

#![feature(allocator_api)]

extern crate alloc;

mod block;
mod context;
mod operation;

pub use block::*;
pub use context::*;
pub use operation::*;
