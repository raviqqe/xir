use xir::{Span, Type};

/// Creates a signed integer type.
pub const fn i64_type(span: Span<'_>) -> Type<'_> {
    Type::new("cl.i64", span)
}

/// Creates a pointer type.
pub const fn pointer_type(span: Span<'_>) -> Type<'_> {
    Type::new("cl.pointer", span)
}
