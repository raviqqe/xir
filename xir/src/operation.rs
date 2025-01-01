use crate::{Allocator, Context, Span, Type, Value};

/// An operation.
#[derive(Debug)]
pub struct Operation<'a> {
    id: &'static str,
    operands: Vec<Value<'a>, Allocator<'a>>,
    value_types: Vec<Type<'a>, Allocator<'a>>,
    span: Span<'a>,
}

impl<'a> Operation<'a> {
    /// Creates an operation.
    pub fn new<const N: usize, const M: usize>(
        context: &'a Context,
        id: &'static str,
        operands: [Value<'a>; N],
        value_types: [Type<'a>; M],
        span: Span<'a>,
    ) -> Self {
        let mut operand_vec = Vec::new_in(context.allocator());
        let mut value_type_vec = Vec::new_in(context.allocator());

        operand_vec.extend(operands);
        value_type_vec.extend(value_types);

        Self {
            id,
            operands: operand_vec,
            value_types: value_type_vec,
            span,
        }
    }

    /// Returns an ID.
    pub fn id(&self) -> &'static str {
        self.id
    }

    /// Returns operands.
    pub fn operands(&self) -> &[Value<'a>] {
        &self.operands
    }

    /// Returns values.
    pub fn value_types(&self) -> &[Type<'a>] {
        &self.value_types
    }

    /// Returns a span.
    pub fn span(&self) -> Span<'a> {
        self.span
    }
}
