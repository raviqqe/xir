use crate::{Span, Type};

/// An argument.
#[derive(Clone, Copy, Debug)]
pub struct BlockArgument<'a> {
    r#type: Type<'a>,
    span: Span<'a>,
}

impl<'a> BlockArgument<'a> {
    /// Creates an argument.
    pub const fn new(r#type: Type<'a>, span: Span<'a>) -> Self {
        Self { r#type, span }
    }

    /// Returns an argument.
    pub const fn r#type(&self) -> &Type {
        &self.r#type
    }

    /// Returns a span.
    pub const fn span(&self) -> Span<'a> {
        self.span
    }
}
