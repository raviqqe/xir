use super::expression::Expression;
use crate::types::Type;

pub trait Instruction {
    type Type: Type;
    type Expression: Expression<Type = Self::Type>;

    fn result(&self) -> Self::Expression;
}
