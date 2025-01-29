use crate::{Allocator, Attribute, Block, Context, OperationValue, Span, Symbol, Type, Value};
use alloc::collections::LinkedList;
use core::cell::RefCell;

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
    attributes: RefCell<LinkedList<(Symbol, Attribute<'a>), Allocator<'a>>>,
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
            attributes: LinkedList::new_in(context.allocator()).into(),
            span,
        }))
    }

    /// Returns an ID.
    pub fn id(&self) -> &'static str {
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

    /// Returns attributes.
    pub fn attribute(&self, name: Symbol) -> Option<Attribute> {
        self.0
            .attributes
            .borrow()
            .iter()
            .find_map(|&(key, value)| (key == name).then_some(value))
    }

    /// Returns attributes.
    pub fn insert_attribute(
        self,
        context: &'a Context<'a>,
        name: &'static str,
        attribute: Attribute<'a>,
    ) -> Self {
        self.0
            .attributes
            .borrow_mut()
            .push_front((Symbol::new(context, name), attribute));

        self
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_attribute() {
        let context = Context::new();
        let operation =
            Operation::new(&context, "foo", &[], &[], [], Span::new(&context, "", 0, 0));

        assert_eq!(operation.attribute(Symbol::new(&context, "bar")), None);

        operation.insert_attribute(&context, "bar", Attribute::I64(0));

        assert_eq!(
            operation.attribute(Symbol::new(&context, "bar")),
            Some(Attribute::I64(0))
        );
    }
}
