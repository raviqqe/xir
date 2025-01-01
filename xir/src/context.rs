use crate::span::Span;
use bumpalo::Bump;
use core::cell::RefCell;
use std::collections::HashSet;

/// An allocator.
pub type Allocator<'a> = &'a Bump;

/// A context.
#[derive(Debug, Default)]
pub struct Context<'a> {
    allocator: Bump,
    spans: RefCell<HashSet<Span<'a>>>,
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
    pub const fn allocator(&self) -> &Bump {
        &self.allocator
    }

    pub(crate) const fn spans(&self) -> &RefCell<HashSet<Span<'a>>> {
        &self.spans
    }
}
