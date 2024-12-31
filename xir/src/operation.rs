/// An operation.
pub trait Operation {
    /// Returns if an operation is of control.
    fn is_control(&self) -> bool;
}
