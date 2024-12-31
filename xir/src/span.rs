use crate::Context;

pub type Position = u32;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Span {
    filename: (),
    start: Position,
    end: Position,
}

impl Span {
    pub fn new<'a>(
        context: &'a mut Context<'a>,
        _filename: &'a str,
        start: Position,
        end: Position,
    ) -> &'a Span {
        let span = context.allocator.alloc(Span {
            filename: (),
            start,
            end,
        });

        context.spans.insert(span);

        span
    }

    pub fn start(&self) -> Position {
        self.start
    }

    pub fn end(&self) -> Position {
        self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let mut context = foo;
    }
}
