use crate::{Allocator, Context, OperationValue, Span, Type, Value};

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
        Self {
            id,
            operands: {
                let mut vec = Vec::new_in(context.allocator());
                vec.extend(operands);
                vec
            },
            value_types: {
                let mut vec = Vec::new_in(context.allocator());
                vec.extend(value_types);
                vec
            },
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

    /// Returns a value.
    pub fn value(&'a self, context: &'a Context<'a>, index: usize) -> Value<'a> {
        OperationValue::new(context, self, index).into()
    }
}
