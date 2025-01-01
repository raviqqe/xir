use crate::{Allocator, BlockArgument, Operation};
use alloc::collections::LinkedList;

/// A block.
#[derive(Debug)]
pub struct Block<'a> {
    arguments: Vec<BlockArgument<'a>, Allocator<'a>>,
    operations: LinkedList<Operation<'a>, Allocator<'a>>,
}

impl<'a> Block<'a> {
    /// Creates a block.
    pub const fn new(
        arguments: Vec<BlockArgument<'a>, Allocator<'a>>,
        operations: LinkedList<Operation<'a>, Allocator<'a>>,
    ) -> Self {
        Self {
            arguments,
            operations,
        }
    }

    /// Returns arguments.
    pub fn arguments(&self) -> &[BlockArgument<'a>] {
        &self.arguments
    }

    /// Returns a reference to operations.
    pub const fn operations(&self) -> &LinkedList<Operation<'a>, Allocator<'a>> {
        &self.operations
    }

    /// Returns a mutable reference to operations.
    pub fn operations_mut(&mut self) -> &mut LinkedList<Operation<'a>, Allocator<'a>> {
        &mut self.operations
    }
}
