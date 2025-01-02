//! Simple control flow operations.

use xir::{Block, Context, Operation, Span, Type};

/// An `if` operation.
pub fn r#if<'a>(context: &'a Context<'a>, types: &[Type<'a>], span: Span<'a>) -> Operation<'a> {
    Operation::new(context, "scf.if", [], types, [], span)
}

/// A `while` operation.
pub fn r#while<'a>(
    context: &'a Context<'a>,
    types: &[Type<'a>],
    before: Block<'a>,
    after: Block<'a>,
    span: Span<'a>,
) -> Operation<'a> {
    Operation::new(context, "scf.while", [], types, [before, after] span)
}

/// A `yield` operation.
pub fn r#yield<'a>(context: &'a Context<'a>, r#type: Type<'a>, span: Span<'a>) -> Operation<'a> {
    Operation::new(context, "scf.while", [], [r#type], [], span)
}
