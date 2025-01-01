use crate::{block_argument::BlockArgument, context::Context, Operation};
use std::cell::RefCell;

/// A value.
#[derive(Clone, Copy, Debug)]
pub enum Value<'a> {
    /// A block argument.
    BlockArgument(BlockArgument<'a>),
    /// An operation value.
    OperationValue(OperationValue<'a>),
}

/// An operation value.
#[derive(Clone, Copy, Debug)]
pub struct OperationValue<'a> {
    inner: &'a RefCell<OperationValueInner<'a>>,
}

#[derive(Debug)]
struct OperationValueInner<'a> {
    operation: &'a Operation,
    index: usize,
}

impl<'a> OperationValue<'a> {
    /// Creates an operation value.
    pub fn new(context: &'a Context, operation: &'a Operation, index: usize) -> Self {
        Self {
            inner: context
                .allocator()
                .alloc(RefCell::new(OperationValueInner { operation, index })),
        }
    }

    /// Returns an operation.
    pub fn operation(&self) -> &Operation {
        self.inner.borrow().operation
    }

    /// Returns an index.
    pub fn index(&self) -> usize {
        self.inner.borrow().index
    }

    /// Replaces a value.
    pub fn replace(&self, operation: &'a Operation, index: usize) {
        *self.inner.borrow_mut() = OperationValueInner { operation, index }
    }
}
