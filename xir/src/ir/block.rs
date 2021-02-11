use super::instruction::Instruction;

#[derive(Clone, Debug, PartialEq)]
pub struct Block<I: Instruction> {
    instructions: Vec<I>,
}

impl<I: Instruction> Block<I> {
    pub fn new(instructions: Vec<I>) -> Self {
        Self { instructions }
    }

    pub fn instructions(&self) -> &[I] {
        &self.instructions
    }
}
