use alloc::collections::LinkedList;

/// A block.
pub struct Block<'c> {
    operations: LinkedList<'a>,
}

impl Block {
    /// Creates a block.
    pub fn new() -> Self {
        Self { operations }
    }
}
