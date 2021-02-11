use super::expression::Expression;

pub trait Instruction {
    type Expression: Expression;

    fn result(&self) -> Self::Expression;
}
