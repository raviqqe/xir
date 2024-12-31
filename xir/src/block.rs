use crate::Operation;
use alloc::{alloc::Allocator, collections::LinkedList};

/// A block.
#[derive(Debug)]
pub struct Block<O: Operation, A: Allocator> {
    operations: LinkedList<O, A>,
}

impl<O: Operation, A: Allocator> Block<O, A> {
    /// Creates a block.
    pub fn new(operations: LinkedList<O>) -> Self {
        Self { operations }
    }

    /// Returns a reference to operations.
    pub fn operations(&self) -> &LinkedList<O> {
        &self.operations
    }

    /// Returns a mutable reference to operations.
    pub fn operations_mut(&mut self) -> &mut LinkedList<O> {
        &mut self.operations
    }
}
