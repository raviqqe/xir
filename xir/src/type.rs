use crate::{Allocator, Span, Symbol};

/// A type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Type<'a> {
    body: TypeBody<'a>,
    span: Span<'a>,
}

/// A type body.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeBody<'a> {
    /// A boolean.
    Bool,
    /// A function.
    Function {
        /// Arguments.
        arguments: &'a [Type<'a>],
        /// Returned values.
        returns: &'a [Type<'a>],
    },
    /// A 16-bit integer.
    I16,
    /// A 32-bit integer.
    I32,
    /// A 64-bit integer.
    I64,
    /// A named type.
    Named(Symbol, &'a Type<'a>),
    /// A tuple.
    Tuple(&'a [Type<'a>]),
}

impl<'a> Type<'a> {
    /// Creates a type.
    pub const fn new(body: TypeBody<'a>, span: Span<'a>) -> Self {
        Self { body, span }
    }

    /// Returns an ID.
    pub const fn body(&self) -> TypeBody {
        self.body
    }

    /// Returns a span.
    pub const fn span(&self) -> Span<'a> {
        self.span
    }
}
