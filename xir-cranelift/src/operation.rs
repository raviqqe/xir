use xir::{Context, Operation, Span, Type, Value};

/// Creates a `iconst` operation.
pub fn iconst<'a>(context: &'a Context<'a>, r#type: Type<'a>, span: Span<'a>) -> Operation<'a> {
    // TODO Set a `value` attribute.
    Operation::new(context, "cl.const", [], [r#type], span)
}

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
