use xir::{Context, Operation, Span, Type, Value};

/// Creates a `iconst` operation.
pub fn iconst<'a>(context: &'a Context<'a>, r#type: Type<'a>, span: Span<'a>) -> Operation<'a> {
    // TODO Set a `value` attribute.
    Operation::new(context, "cl.const", &[], &[r#type], vec![], span)
}

/// Creates an `iadd` operation.
pub fn iadd<'a>(
    context: &'a Context<'a>,
    lhs: Value<'a>,
    rhs: Value<'a>,
    r#type: Type<'a>,
    span: Span<'a>,
) -> Operation<'a> {
    Operation::new(context, "cl.iadd", &[lhs, rhs], &[r#type], vec![], span)
}

/// Creates a `load` operation.
pub fn load<'a>(
    context: &'a Context<'a>,
    r#type: Type<'a>,
    pointer: Value<'a>,
    span: Span<'a>,
) -> Operation<'a> {
    Operation::new(context, "cl.load", &[pointer], &[r#type], vec![], span)
}

/// Creates a `store` operation.
pub fn store<'a>(
    context: &'a Context<'a>,
    value: Value<'a>,
    pointer: Value<'a>,
    span: Span<'a>,
) -> Operation<'a> {
    Operation::new(context, "cl.store", &[value, pointer], &[], vec![], span)
}
