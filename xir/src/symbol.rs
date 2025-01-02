use crate::Context;

/// A symbol.
#[derive(Debug, Eq)]
pub struct Symbol(&'static str);

impl Symbol {
    /// Creates a symbol.
    pub fn new(context: &Context, name: &'static str) -> Self {
        Self(name)
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ptr() == other.0.as_ptr()
    }
}
