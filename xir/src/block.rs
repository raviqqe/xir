use crate::Operation;
use alloc::{alloc::Allocator, collections::LinkedList};

/// A block.
#[derive(Debug)]
pub struct Block<O: Operation<A>, A: Allocator> {
    operations: LinkedList<O, A>,
}

impl<O: Operation<A>, A: Allocator> Block<O, A> {
    /// Creates a block.
    pub fn new(operations: LinkedList<O, A>) -> Self {
        Self { operations }
    }

    /// Returns a reference to operations.
    pub fn operations(&self) -> &LinkedList<O, A> {
        &self.operations
    }

    /// Returns a mutable reference to operations.
    pub fn operations_mut(&mut self) -> &mut LinkedList<O, A> {
        &mut self.operations
    }
}
