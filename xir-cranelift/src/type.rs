use xir::{Span, Type};

/// Creates a signed integer type.
pub fn signed_integer_type(span: Span<'a>) -> Type {
    Type::new("cl.i32", span)
}
