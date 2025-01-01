use crate::Span;

/// A type.
#[derive(Clone, Copy, Debug)]
pub struct Type<'a> {
    id: &'static str,
    span: Span<'a>,
}

impl<'a> Type<'a> {
    /// Creates a type.
    pub fn new(id: &'static str, span: Span<'a>) -> Self {
        Self { id, span }
    }

    /// Returns an ID.
    pub fn id(&self) -> &'static str {
        self.id
    }

    /// Returns a span.
    pub fn span(&self) -> Span<'a> {
        self.span
    }
}
