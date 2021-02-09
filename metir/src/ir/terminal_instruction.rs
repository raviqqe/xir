use super::branch::Branch;
use super::expression::Expression;
use super::return_::Return;
use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum TerminalInstruction<T: Type, E: Expression<T>> {
    Branch(Branch<T, E>),
    Return(Return<T, E>),
    Unreachable,
}

impl<T: Type, E: Expression<T>> TerminalInstruction<T, E> {
    pub fn to_branch(&self) -> Option<&Branch<T, E>> {
        if let TerminalInstruction::Branch(branch) = self {
            Some(branch)
        } else {
            None
        }
    }

    pub fn to_return(&self) -> Option<&Return<T, E>> {
        if let TerminalInstruction::Return(return_) = self {
            Some(return_)
        } else {
            None
        }
    }
}

impl<T: Type, E: Expression<T>> From<Branch<T, E>> for TerminalInstruction<T, E> {
    fn from(branch: Branch<T, E>) -> Self {
        Self::Branch(branch)
    }
}

impl<T: Type, E: Expression<T>> From<Return<T, E>> for TerminalInstruction<T, E> {
    fn from(return_: Return<T, E>) -> Self {
        Self::Return(return_)
    }
}
