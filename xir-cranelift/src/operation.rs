use crate::r#type::Type;
use xir::Operation;

/// Creates an `iadd` operation.
pub fn iadd(lhs: Value<'a>, rhs: Value<'a>, span: Span<'a>) -> Operation<'a> {
    Operation::new("cl.iadd", [lhs, rhs], span)
}
