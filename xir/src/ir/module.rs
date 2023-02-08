use super::{
    expression::Expression, function_declaration::FunctionDeclaration,
    function_definition::FunctionDefinition, instruction::Instruction,
    variable_declaration::VariableDeclaration, variable_definition::VariableDefinition,
};
use crate::types::{FunctionType, Type};

#[derive(Clone, Debug, PartialEq)]
pub struct Module<
    T: Type,
    F: FunctionType<Type = T>,
    E: Expression<Type = T>,
    I: Instruction<Expression = E>,
> {
    variable_declarations: Vec<VariableDeclaration<T>>,
    function_declarations: Vec<FunctionDeclaration<T, F>>,
    variable_definitions: Vec<VariableDefinition<T, E>>,
    function_definitions: Vec<FunctionDefinition<F, I>>,
}

impl<
        T: Type,
        F: FunctionType<Type = T>,
        E: Expression<Type = T>,
        I: Instruction<Expression = E>,
    > Module<T, F, E, I>
{
    pub fn new(
        variable_declarations: Vec<VariableDeclaration<T>>,
        function_declarations: Vec<FunctionDeclaration<T, F>>,
        variable_definitions: Vec<VariableDefinition<T, E>>,
        function_definitions: Vec<FunctionDefinition<F, I>>,
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

    pub fn function_definitions(&self) -> &[FunctionDefinition<F, I>] {
        &self.function_definitions
    }
}
