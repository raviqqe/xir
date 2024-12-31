use std::alloc::Allocator;

pub struct Location<'a, A: Allocator> {
    filename: &'a str,
    position: usize,
}

pub struct Span {
    start: Location,
    end: Location,
}

impl Span {
    pub fn new(context: &Context, start: Location, end: Location) -> Span {
        Span { start, end }
    }

    pub fn start(&self) -> Location {
        self.start
    }

    pub fn end(&self) -> Location {
        self.end
    }
}
