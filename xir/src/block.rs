use crate::Operation;
use alloc::collections::LinkedList;
use core::alloc::Allocator;

/// A block.
#[derive(Debug)]
pub struct Block<O: Operation<A>, A: Allocator> {
    arguments: Vec<O::Type, A>,
    operations: LinkedList<O, A>,
}

impl<O: Operation<A>, A: Allocator> Block<O, A> {
    /// Creates a block.
    pub const fn new(arguments: Vec<O::Type, A>, operations: LinkedList<O, A>) -> Self {
        Self {
            arguments,
            operations,
        }
    }

    /// Returns arguments.
    pub fn arguments(&self) -> &[O::Type] {
        &self.arguments
    }

    /// Returns a reference to operations.
    pub const fn operations(&self) -> &LinkedList<O, A> {
        &self.operations
    }

    /// Returns a mutable reference to operations.
    pub fn operations_mut(&mut self) -> &mut LinkedList<O, A> {
        &mut self.operations
    }
}
