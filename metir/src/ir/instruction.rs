use super::expression::Expression;
use crate::types::Type;

pub trait Instruction<T: Type, E: Expression<T>> {
    fn run(&self, arguments: Vec<E>) -> Option<E>;
}
