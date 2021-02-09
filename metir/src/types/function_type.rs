use super::type_::Type;

pub trait FunctionType: Type {
    type Type: Type;

    fn arguments(&self) -> &[Self::Type];
    fn result(&self) -> &Self::Type;
}
