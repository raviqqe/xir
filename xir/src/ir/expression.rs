use crate::types::Type;

pub trait Expression {
    type Type: Type;

    fn type_(&self) -> &Self::Type;
}
