use xir::{Span, Type};

/// Creates a signed integer type.
pub fn i64_type<'a>(span: Span<'a>) -> Type<'a> {
    Type::new("cl.i64", span)
}
