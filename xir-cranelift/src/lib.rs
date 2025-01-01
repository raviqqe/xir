//! Cranelift intermediate representation for XIR.

#![feature(allocator_api)]

mod operation;
mod r#type;

pub use operation::*;
pub use r#type::*;

#[cfg(test)]
mod tests {
    use super::*;
    use xir::{Block, BlockArgument, Context, Span};

    #[test]
    fn test() {
        let context = Context::new();
        let span = Span::new(&context, "", 0, 0);
        let r#type = i64_type(span);
        let mut block = Block::new(&context, [BlockArgument::new(r#type, span)]);

        block
            .operations_mut()
            .push_back(iconst(&context, r#type, span));
        let value = block.operations().back().unwrap().value(&context, 0);

        block
            .operations_mut()
            .push_back(iadd(&context, value, value, r#type, span));
    }
}
