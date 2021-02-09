use super::expression::Expression;
use crate::types::Type;

pub trait Instruction {
    type Type: Type;
    type Expression: Expression;

    fn result(&self) -> Option<Self::Expression>;
}
