use crate::r#type::Type;
use core::alloc::Allocator;
use xir::{Block, Context};

/// An operation.
pub enum Operation<'a, A: Allocator> {
    IAdd(IAdd<'a, A>),
}

pub struct IAdd<'a, A: Allocator> {
    arguments: Vec<()>,
    results: Vec<()>,
}

impl<'a, A: Allocator> IAdd<'a, A> {
    pub fn new(context: &Context) -> Self {
        Self {
            arguments: Default::default(),
            results: Default::default(),
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
