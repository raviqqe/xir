//! Simple control flow operations.

/// An `if` operation.
pub fn r#if<'a>(context: &'a Context<'a>, r#type: Type<'a>, span: Span<'a>) -> Operation<'a> {
    Operation::new(context, "cl.const", [], [r#type], span)
}

/// A `while` operation.
pub fn r#while<'a>(context: &'a Context<'a>, r#type: Type<'a>, span: Span<'a>) -> Operation<'a> {
    Operation::new(context, "cl.const", [], [r#type], span)
}
