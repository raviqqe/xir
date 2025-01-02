use crate::{Allocator, Block, Context, OperationValue, Span, Symbol, Type, Value};

/// An operation.
#[derive(Clone, Copy, Debug)]
pub struct Operation<'a>(&'a OperationInner<'a>);

/// An operation inner.
#[derive(Debug)]
struct OperationInner<'a> {
    id: Symbol,
    operands: Vec<Value<'a>, Allocator<'a>>,
    value_types: Vec<Type<'a>, Allocator<'a>>,
    blocks: Vec<Block<'a>, Allocator<'a>>,
    span: Span<'a>,
}

impl<'a> Operation<'a> {
    /// Creates an operation.
    pub fn new(
        context: &'a Context,
        id: &'static str,
        operands: &[Value<'a>],
        value_types: &[Type<'a>],
        blocks: impl IntoIterator<Item = Block<'a>>,
        span: Span<'a>,
    ) -> Self {
        Self(context.allocator().alloc(OperationInner {
            id: Symbol::new(context, id),
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
            blocks: {
                let mut vec = Vec::new_in(context.allocator());
                vec.extend(blocks);
                vec
            },
            span,
        }))
    }

    /// Returns an ID.
    pub const fn id(&self) -> &'static str {
        self.0.id.name()
    }

    /// Returns operands.
    pub fn operands(&self) -> &[Value<'a>] {
        &self.0.operands
    }

    /// Returns values.
    pub fn value_types(&self) -> &[Type<'a>] {
        &self.0.value_types
    }

    /// Returns blocks.
    pub fn blocks(&self) -> &[Block<'a>] {
        &self.0.blocks
    }

    /// Returns a span.
    pub const fn span(&self) -> Span<'a> {
        self.0.span
    }

    /// Returns a value.
    pub fn value(self, context: &'a Context<'a>, index: usize) -> Value<'a> {
        OperationValue::new(context, self, index).into()
    }
}
