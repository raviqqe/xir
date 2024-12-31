use alloc::collections::LinkedList;

/// A block.
pub struct Block<'c> {
    operations: LinkedList<'a, Operation>,
}

impl Block {
    /// Creates a block.
    pub fn new() -> Self {
        Self { operations }
    }

    /// Creates a block.
    pub fn operations(&self) -> Self {
        self.operations
    }

    pub fn operations_mut(&mut self) -> &mut LinkedList<Operation> {
        Self { operations }
    }
}
