use super::type_::Type;

pub trait Function<T: Type>: Type + Into<T> {
    fn arguments(&self) -> &[T];
    fn result(&self) -> &T;
}
