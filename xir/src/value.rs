use crate::{argument::Argument, Operation, Type};

/// A value.
#[derive(Clone, Copy, Debug)]
pub enum Value<'a, O: Operation, T: Type> {
    /// A block argument.
    BlockArgument(Argument<'a, T>),
    /// An operation value.
    OperationValue(&'a OperationValue<'a, O>),
}

/// An operation value.
#[derive(Clone, Copy, Debug)]
pub struct OperationValue<'a, O: Operation> {
    operation: &'a O,
    index: usize,
}

impl<'a, O: Operation> OperationValue<'a, O> {
    /// Creates an operation value.
    pub fn new(operation: &'a O, index: usize) -> Self {
        Self { operation, index }
    }

    /// Returns an operation.
    pub fn operation(&self) -> &'a O {
        self.operation
    }

    /// Returns an index.
    pub fn index(&self) -> usize {
        self.index
    }
}
