use crate::types::{FunctionType, Type};

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDeclaration<T: Type, F: FunctionType<Type = T>> {
    name: String,
    type_: F,
}

impl<T: Type, F: FunctionType<Type = T>> FunctionDeclaration<T, F> {
    pub fn new(name: impl Into<String>, type_: F) -> Self {
        Self {
            name: name.into(),
            type_,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn type_(&self) -> &F {
        &self.type_
    }
}
