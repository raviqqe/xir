use bumpalo::Bump;
use core::alloc::Allocator;

/// A context.
#[derive(Debug, Default)]
pub struct Context {
    allocator: Bump,
}

impl Context {
    /// Creates a context.
    pub fn new() -> Self {
        Self {
            allocator: Bump::new(),
        }
    }

    /// Returns an allocator.
    pub fn allocator(&self) -> impl Allocator + '_ {
        &self.allocator
    }
}
