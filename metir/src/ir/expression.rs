use crate::types::Type;

pub trait Expression<T: Type> {
    fn type_(&self) -> &T;
}
