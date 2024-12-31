use crate::Type;

/// An argument.
pub struct Argument<T: Type> {
    r#type: T,
}

impl<T: Type> Argument<T> {
    /// Creates an argument.
    pub fn new(r#type: T) -> Self {
        Self { r#type }
    }

    /// Returns an argument.
    pub fn r#type(&self) -> &T {
        &self.r#type
    }
}
