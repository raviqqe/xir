use super::{block::Block, instruction::Instruction};
use crate::types::FunctionType;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDefinition<F: FunctionType, I: Instruction> {
    name: String,
    arguments: Vec<String>,
    body: Block<I>,
    type_: F,
    global: bool,
}

impl<F: FunctionType, I: Instruction> FunctionDefinition<F, I> {
    pub fn new(
        name: impl Into<String>,
        arguments: Vec<String>,
        body: Block<I>,
        type_: impl Into<F>,
        global: bool,
    ) -> Self {
        Self {
            name: name.into(),
            arguments,
            body,
            type_: type_.into(),
            global,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    pub fn body(&self) -> &Block<I> {
        &self.body
    }

    pub fn type_(&self) -> &F {
        &self.type_
    }

    pub fn is_global(&self) -> bool {
        self.global
    }
}
