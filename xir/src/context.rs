use bumpalo::Bump;
use std::{cell::RefCell, collections::HashSet};

use crate::span::Span;

/// A context.
#[derive(Debug, Default)]
pub struct Context<'a> {
    pub(crate) allocator: Bump,
    pub(crate) spans: RefCell<HashSet<&'a Span<'a>>>,
}

impl<'a> Context<'a> {
    /// Creates a context.
    pub fn new() -> Self {
        Self {
            allocator: Bump::new(),
            spans: Default::default(),
        }
    }

    /// Returns an allocator.
    pub fn allocator(&self) -> &Bump {
        &self.allocator
    }
}
