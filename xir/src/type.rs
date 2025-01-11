use crate::{Span, Symbol};

/// A type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Type<'a> {
    body: TypeBody<'a>,
    span: Span<'a>,
}

/// A type body.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeBody<'a> {
    Bool,
    Function(Symbol<'a>, &'a Type<'a>),
    I16,
    I32,
    I64,
    Named(Symbol<'a>, &'a Type<'a>),
    Tuple(Symbol<'a>, &'a Type<'a>),
}

impl<'a> Type<'a> {
    /// Creates a type.
    pub const fn new(body: TypeBody<'a>, span: Span<'a>) -> Self {
        Self { body, span }
    }

    /// Returns an ID.
    pub const fn body(&self) -> TypeBody {
        self.id
    }

    /// Returns a span.
    pub const fn span(&self) -> Span<'a> {
        self.span
    }
}
