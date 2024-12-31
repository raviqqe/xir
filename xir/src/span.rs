use crate::Context;

/// A position.
pub type Position = u32;

/// A span.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Span {
    filename: (),
    start: Position,
    end: Position,
}

impl Span {
    /// Creates a span.
    pub fn new<'a>(
        context: &'a Context<'a>,
        _filename: &'a str,
        start: Position,
        end: Position,
    ) -> &'a Span {
        let span = context.allocator.alloc(Span {
            filename: (),
            start,
            end,
        });

        context.spans.borrow_mut().insert(span);

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
    fn foo() {
        let context = Context::new();

        Span::new(&context, "foo.rs", 0, 1);
    }
}
