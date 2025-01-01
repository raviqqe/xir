use crate::r#type::Type;
use core::alloc::Allocator;
use xir::{Block, Context, Span};

/// An operation.
pub enum Operation<'a, A: Allocator> {
    IAdd(IAdd<'a, A>),
}

pub struct IAdd<'a, A: Allocator> {
    arguments: Vec<(), A>,
    results: Vec<(), A>,
}

impl<'a, A: Allocator> IAdd<'a, A> {
    pub fn new(context: &Context, lhs: (), rhs: (), span: Span) -> Self {
        Self {
            arguments: Vec::new_in(context.allocator()),
            results: Vec::new_in(context.allocator()),
        }
    }
}

impl<'a, A: Allocator> xir::Operation<A> for IAdd<'a, A> {
    type Type = Type;

    type InnerOperation = Operation<'a, A>;

    fn blocks<'b>(&'b self) -> impl Iterator<Item = &'b Block<'b, Self::InnerOperation, A>>
    where
        A: 'b,
    {
        [].iter()
    }

    fn is_control(&self) -> bool {
        false
    }
}
