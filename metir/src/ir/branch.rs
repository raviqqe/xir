use super::expression::Expression;
use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct Branch<T: Type, E: Expression<T>> {
    type_: T,
    expression: E,
}

impl<T: Type, E: Expression<T>> Branch<T, E> {
    pub fn new(type_: impl Into<T>, expression: impl Into<E>) -> Self {
        Self {
            type_: type_.into(),
            expression: expression.into(),
        }
    }

    pub fn type_(&self) -> &T {
        &self.type_
    }

    pub fn expression(&self) -> &E {
        &self.expression
    }
}
