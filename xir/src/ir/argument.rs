use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct Argument<T: Type> {
    name: String,
    type_: T,
}

impl<T: Type> Argument<T> {
    pub fn new(name: impl Into<String>, type_: impl Into<T>) -> Self {
        Self {
            name: name.into(),
            type_: type_.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn type_(&self) -> &T {
        &self.type_
    }
}
