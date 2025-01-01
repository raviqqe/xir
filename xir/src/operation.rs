use crate::{Allocator, Context, OperationValue, Span, Type, Value};

/// An operation.
#[derive(Clone, Copy, Debug)]
pub struct Operation<'a>(&'a OperationInner<'a>);

/// An operation inner.
#[derive(Debug)]
struct OperationInner<'a> {
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
        Self(context.allocator().alloc(OperationInner {
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
        }))
    }

    /// Returns an ID.
    pub fn id(&self) -> &'static str {
        self.0.id
    }

    /// Returns operands.
    pub fn operands(&self) -> &[Value<'a>] {
        &self.0.operands
    }

    /// Returns values.
    pub fn value_types(&self) -> &[Type<'a>] {
        &self.0.value_types
    }

    /// Returns a span.
    pub fn span(&self) -> Span<'a> {
        self.0.span
    }

    /// Returns a value.
    pub fn value(self, context: &'a Context<'a>, index: usize) -> Value<'a> {
        OperationValue::new(context, self, index).into()
    }
}
