use bumpalo::Bump;
use std::collections::HashSet;

use crate::span::Span;

/// A context.
#[derive(Debug, Default)]
pub struct Context<'a> {
    pub(crate) allocator: Bump,
    pub(crate) spans: HashSet<&'a Span>,
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

    /// Returns a mutable reference to spans.
    pub fn spans_mut(&mut self) -> &mut HashSet<&'a Span> {
        &mut self.spans
    }
}
