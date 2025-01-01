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

impl<'a> OperationValue<'a> {
    /// Creates an operation value.
    pub fn new(context: &'a Context, operation: &'a Operation, index: usize) -> Self {
        Self {
            inner: context
                .allocator()
                .alloc(RefCell::new(OperationValueInner { operation, index })),
        }
    }

    /// Replaces a value.
    pub fn replace(&self, operation: &'a Operation, index: usize) {
        *self.inner.borrow_mut() = OperationValueInner { operation, index }
    }
}

/// An operation value.
#[derive(Debug)]
pub struct OperationValueInner<'a> {
    operation: &'a Operation,
    index: usize,
}

impl<'a> OperationValueInner<'a> {
    /// Creates an operation value.
    pub fn new(operation: &'a Operation, index: usize) -> Self {
        Self { operation, index }
    }

    /// Returns an operation.
    pub fn operation(&self) -> &'a Operation {
        self.operation
    }

    /// Returns an index.
    pub fn index(&self) -> usize {
        self.index
    }
}
