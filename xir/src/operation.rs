use crate::{block::Block, Type};
use core::alloc::Allocator;

/// An operation.
pub trait Operation<A: Allocator> {
    /// A type.
    type Type: Type;

    /// An inner operation.
    type InnerOperation: Operation<A>;

    /// Returns if an operation is of control.
    fn blocks<'a>(&'a self) -> impl Iterator<Item = &'a Block<'a, Self::InnerOperation, A>>
    where
        A: 'a;

    /// Returns if an operation is of control.
    fn is_control(&self) -> bool;
}
