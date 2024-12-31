use crate::Operation;
use alloc::{alloc::Allocator, collections::LinkedList};

/// A block.
#[derive(Debug)]
pub struct Block<A: Allocator> {
    operations: LinkedList<Operation, A>,
}

impl Block {
    /// Creates a block.
    pub fn new(operations: LinkedList<Operation>) -> Self {
        Self { operations }
    }

    /// Creates a block.
    pub fn operations(&self) -> &LinkedList<Operation> {
        &self.operations
    }

    pub fn operations_mut(&mut self) -> &mut LinkedList<Operation> {
        &mut self.operations
    }
}
