use crate::{Allocator, Context, OperationValue, Value};

/// An operation.
#[derive(Debug)]
pub struct Operation<'a> {
    id: &'static str,
    operands: Vec<Value<'a>, Allocator<'a>>,
    values: Vec<OperationValue<'a>, Allocator<'a>>,
}

impl<'a> Operation<'a> {
    /// Creates an operation.
    pub fn new<const N: usize, const M: usize>(
        context: &'a Context,
        id: &'static str,
        operands: [Value<'a>; N],
        values: [OperationValue<'a>; M],
    ) -> Self {
        let mut operand_vec = Vec::new_in(context.allocator());
        let mut value_vec = Vec::new_in(context.allocator());

        operand_vec.extend(operands);
        value_vec.extend(values);

        Self {
            id,
            operands: operand_vec,
            values: value_vec,
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
    pub fn values(&self) -> &[OperationValue<'a>] {
        &self.values
    }
}
