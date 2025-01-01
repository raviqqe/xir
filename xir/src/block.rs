use crate::{Allocator, Argument, Operation};
use alloc::collections::LinkedList;

/// A block.
#[derive(Debug)]
pub struct Block<'a, O: Operation> {
    arguments: Vec<Argument<'a, O::Type>, Allocator<'a>>,
    operations: LinkedList<O, Allocator<'a>>,
}

impl<'a, O: Operation> Block<'a, O> {
    /// Creates a block.
    pub const fn new(
        arguments: Vec<Argument<'a, O::Type>, Allocator<'a>>,
        operations: LinkedList<O, Allocator<'a>>,
    ) -> Self {
        Self {
            arguments,
            operations,
        }
    }

    /// Returns arguments.
    pub fn arguments(&self) -> &[Argument<'a, O::Type>] {
        &self.arguments
    }

    /// Returns a reference to operations.
    pub const fn operations(&self) -> &LinkedList<O, Allocator<'a>> {
        &self.operations
    }

    /// Returns a mutable reference to operations.
    pub fn operations_mut(&mut self) -> &mut LinkedList<O, Allocator<'a>> {
        &mut self.operations
    }
}
