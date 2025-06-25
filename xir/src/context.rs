use crate::{Span, Symbol};
use bumpalo::Bump;
use core::cell::RefCell;
use dashmap::DashMap;
use std::collections::HashSet;

/// An allocator.
pub(crate) type Allocator<'a> = &'a Bump;

/// A context.
#[derive(Debug, Default)]
pub struct Context<'a> {
    allocator: Bump,
    spans: RefCell<HashSet<Span<'a>>>,
    symbols: DashMap<&'static str, Symbol>,
}

impl<'a> Context<'a> {
    /// Creates a context.
    pub fn new() -> Self {
        Self {
            // TODO Split allocators by types.
            allocator: Bump::new(),
            spans: Default::default(),
            symbols: Default::default(),
        }
    }

    /// Returns an allocator.
    pub(crate) const fn allocator(&self) -> Allocator<'_> {
        &self.allocator
    }

    pub(crate) const fn spans(&self) -> &RefCell<HashSet<Span<'a>>> {
        &self.spans
    }

    pub(crate) const fn symbols(&self) -> &DashMap<&'static str, Symbol> {
        &self.symbols
    }
}
