use std::alloc::Allocator;
use xir::Context;

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

impl xir::Operation for IAdd {}
