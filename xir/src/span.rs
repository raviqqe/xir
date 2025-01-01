use crate::Context;

/// A position.
pub type Position = u32;

/// A span.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Span<'a>(&'a SpanInner<'a>);

#[derive(Debug, Eq, Hash, PartialEq)]
struct SpanInner<'a> {
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
    ) -> Self {
        let span = Span(context.allocator().alloc(SpanInner {
            filename,
            start,
            end,
        }));

        context.spans().borrow_mut().insert(span);

        span
    }

    /// Returns a filename.
    pub const fn filename(&self) -> &'a str {
        self.0.filename
    }

    /// Returns a start position.
    pub const fn start(&self) -> Position {
        self.0.start
    }

    /// Returns an end position.
    pub const fn end(&self) -> Position {
        self.0.end
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
