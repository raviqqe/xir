use crate::Context;
use core::alloc::Allocator;

pub type Position = u32;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Span {
    filename: (),
    start: Position,
    end: Position,
}

impl Span {
    pub fn new<'a>(
        context: &'a Context<'a>,
        filename: &'a str,
        start: Position,
        end: Position,
    ) -> &'a Span {
        let span = context.allocator().alloc(Span {
            filename: (),
            start,
            end,
        });

        //context.spans_mut().push();

        span
    }

    pub fn start(&self) -> Position {
        self.start
    }

    pub fn end(&self) -> Position {
        self.end
    }
}
