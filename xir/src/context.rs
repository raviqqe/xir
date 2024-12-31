use bumpalo::Bump;
use core::alloc::Allocator;

pub struct Context {
    allocator: Bump,
}

impl Context {
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
