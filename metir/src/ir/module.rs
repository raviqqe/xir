use super::expression::Expression;
use super::function_declaration::FunctionDeclaration;
use super::function_definition::FunctionDefinition;
use super::instruction::Instruction;
use super::variable_declaration::VariableDeclaration;
use super::variable_definition::VariableDefinition;
use crate::types::{self, Type};

#[derive(Clone, Debug, PartialEq)]
pub struct Module<T: Type, F: types::Function<T>, E: Expression<T>, I: Instruction<T, E>> {
    variable_declarations: Vec<VariableDeclaration<T>>,
    function_declarations: Vec<FunctionDeclaration<T, F>>,
    variable_definitions: Vec<VariableDefinition<T, E>>,
    function_definitions: Vec<FunctionDefinition<T, F, E, I>>,
}

impl<T: Type, F: types::Function<T>, E: Expression<T>, I: Instruction<T, E>> Module<T, F, E, I> {
    pub fn new(
        variable_declarations: Vec<VariableDeclaration<T>>,
        function_declarations: Vec<FunctionDeclaration<T, F>>,
        variable_definitions: Vec<VariableDefinition<T, E>>,
        function_definitions: Vec<FunctionDefinition<T, F, E, I>>,
    ) -> Self {
        Self {
            variable_declarations,
            function_declarations,
            variable_definitions,
            function_definitions,
        }
    }

    pub fn variable_declarations(&self) -> &[VariableDeclaration<T>] {
        &self.variable_declarations
    }

    pub fn function_declarations(&self) -> &[FunctionDeclaration<T, F>] {
        &self.function_declarations
    }

    pub fn variable_definitions(&self) -> &[VariableDefinition<T, E>] {
        &self.variable_definitions
    }

    pub fn function_definitions(&self) -> &[FunctionDefinition<T, F, E, I>] {
        &self.function_definitions
    }
}
