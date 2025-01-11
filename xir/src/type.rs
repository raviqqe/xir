use crate::Span;

/// A type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type<'a> {
    Named (  Symbol<'a>,  &'a Type<'a>, span: Span<'a> ),
    I64 (   span: Span<'a> ),
}

impl<'a> Type<'a> {
    /// Creates a type.
    pub const fn new(id: &'static str, span: Span<'a>) -> Self {
        Self { id, span }
    }

    /// Returns an ID.
    pub const fn id(&self) -> &'static str {
        self.id
    }

    /// Returns a span.
    pub const fn span(&self) -> Span<'a> {
        self.span
    }
}
