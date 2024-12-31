use crate::Context;

/// A position.
pub type Position = u32;

/// A span.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Span<'a> {
    filename: &'a str,
    start: Position,
    end: Position,
}

impl<'a> Span<'a> {
    /// Creates a span.
    pub fn new(
        context: &'a Context<'a>,
        filename: &'a str,
        start: Position,
        end: Position,
    ) -> &'a Span<'a> {
        let span = context.allocator().alloc(Span {
            filename,
            start,
            end,
        });

        context.spans().borrow_mut().insert(span);

        span
    }

    /// Returns a start position.
    pub fn start(&self) -> Position {
        self.start
    }

    /// Returns an end position.
    pub fn end(&self) -> Position {
        self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let context = Context::new();

        Span::new(&context, "foo.rs", 0, 1);
    }
}
