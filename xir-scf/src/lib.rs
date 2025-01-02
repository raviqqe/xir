//! Simple control flow operations.

use xir::Block;

/// An `if` operation.
pub fn r#if<'a>(context: &'a Context<'a>, r#type: Type<'a>, span: Span<'a>) -> Operation<'a> {
    Operation::new(context, "scf.const", [], [r#type], span)
}

/// A `while` operation.
pub fn r#while<'a>(
    context: &'a Context<'a>,
    r#type: Type<'a>,
    before: Block<'a>,
    after: Block<'a>,
    span: Span<'a>,
) -> Operation<'a> {
    Operation::new(context, "scf.while", [], [r#type], span)
}
