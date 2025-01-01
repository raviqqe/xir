use xir::{Context, Operation, Span, Type, Value};

/// Creates an `iadd` operation.
pub fn iadd<'a>(
    context: &'a Context<'a>,
    lhs: Value<'a>,
    rhs: Value<'a>,
    r#type: Type<'a>,
    span: Span<'a>,
) -> Operation<'a> {
    Operation::new(context, "cl.iadd", [lhs, rhs], [r#type], span)
}
