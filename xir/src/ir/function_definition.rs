use super::block::Block;
use super::expression::Expression;
use super::instruction::Instruction;
use crate::types::{FunctionType, Type};

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDefinition<
    T: Type,
    F: FunctionType,
    E: Expression<Type = T>,
    I: Instruction<Type = T, Expression = E>,
> {
    name: String,
    arguments: Vec<String>,
    body: Block<T, E, I>,
    type_: F,
    global: bool,
}

impl<
        T: Type,
        F: FunctionType,
        E: Expression<Type = T>,
        I: Instruction<Type = T, Expression = E>,
    > FunctionDefinition<T, F, E, I>
{
    pub fn new(
        name: impl Into<String>,
        arguments: Vec<String>,
        body: Block<T, E, I>,
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

    pub fn body(&self) -> &Block<T, E, I> {
        &self.body
    }

    pub fn type_(&self) -> &F {
        &self.type_
    }

    pub fn is_global(&self) -> bool {
        self.global
    }
}
