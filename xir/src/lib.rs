//! Blah-blah intermediate representation.

#![feature(allocator_api)]

extern crate alloc;

mod block;
mod operation;

pub use block::*;
pub use operation::*;
