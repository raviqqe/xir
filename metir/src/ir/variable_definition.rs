use super::expression::Expression;
use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct VariableDefinition<T: Type, E: Expression<T>> {
    name: String,
    body: E,
    type_: T,
    mutable: bool,
    global: bool,
}

impl<T: Type, E: Expression<T>> VariableDefinition<T, E> {
    pub fn new(
        name: impl Into<String>,
        body: impl Into<E>,
        type_: impl Into<T>,
        mutable: bool,
        global: bool,
    ) -> Self {
        Self {
            name: name.into(),
            body: body.into(),
            type_: type_.into(),
            mutable,
            global,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn body(&self) -> &E {
        &self.body
    }

    pub fn type_(&self) -> &T {
        &self.type_
    }

    pub fn is_mutable(&self) -> bool {
        self.mutable
    }

    pub fn is_global(&self) -> bool {
        self.global
    }
}
