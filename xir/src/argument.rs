use crate::{Span, Type};

/// An argument.
#[derive(Clone, Copy, Debug)]
pub struct Argument<'a, T: Type> {
    r#type: T,
    span: Span<'a>,
}

impl<'a, T: Type> Argument<'a, T> {
    /// Creates an argument.
    pub const fn new(r#type: T, span: Span<'a>) -> Self {
        Self { r#type, span }
    }

    /// Returns an argument.
    pub const fn r#type(&self) -> &T {
        &self.r#type
    }

    /// Returns a span.
    pub const fn span(&self) -> Span<'a> {
        self.span
    }
}
