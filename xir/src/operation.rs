use crate::Type;

/// An operation.
pub trait Operation {
    /// A type.
    type Type: Type;

    /// An inner operation.
    type InnerOperation: Operation;

    /// Returns if an operation is of control.
    fn is_control(&self) -> bool;
}
