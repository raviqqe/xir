use super::expression::Expression;
use super::instruction::Instruction;
use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct Block<T: Type, E: Expression, I: Instruction<Type = T, Expression = E>> {
    instructions: Vec<I>,
}

impl<T: Type, E: Expression, I: Instruction<Type = T, Expression = E>> Block<T, E, I> {
    pub fn new(instructions: Vec<I>) -> Self {
        Self { instructions }
    }

    pub fn instructions(&self) -> &[I] {
        &self.instructions
    }
}
