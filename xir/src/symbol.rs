use crate::Context;

/// A symbol.
#[derive(Clone, Copy, Debug, Eq)]
pub struct Symbol(&'static str);

impl Symbol {
    /// Creates a symbol.
    pub fn new(context: &Context, name: &'static str) -> Self {
        *context
            .symbols()
            .entry(name)
            .or_insert_with(|| Self(name))
            .value()
    }

    /// Returns a name.
    pub const fn name(&self) -> &'static str {
        self.0
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ptr() == other.0.as_ptr()
    }
}
