//! Cranelift intermediate representation for XIR.

#![feature(allocator_api)]

mod operation;
mod r#type;

pub use operation::*;
pub use r#type::*;

#[cfg(test)]
mod tests {
    use super::*;
    use xir::{Attribute, Block, BlockArgument, Context, Span};

    #[test]
    fn add_integers() {
        let context = Context::new();
        let span = Span::new(&context, "", 0, 0);
        let r#type = i64_type(span);
        let mut block = Block::new(&context, [BlockArgument::new(r#type, span)]);
        let argument = block.arguments()[0].into();

        block
            .operations_mut()
            .push_back(iconst(&context, r#type, Attribute::I64(0), span));
        let value = block.operations().back().unwrap().value(&context, 0);

        block
            .operations_mut()
            .push_back(iadd(&context, value, argument, r#type, span));
    }

    #[test]
    fn load_integer() {
        let context = Context::new();
        let span = Span::new(&context, "", 0, 0);
        let integer_type = i64_type(span);
        let pointer_type = super::pointer_type(span);
        let mut block = Block::new(&context, [BlockArgument::new(pointer_type, span)]);
        let pointer = block.arguments()[0].into();

        block
            .operations_mut()
            .push_back(load(&context, integer_type, pointer, span));
    }

    #[test]
    fn store_integer() {
        let context = Context::new();
        let span = Span::new(&context, "", 0, 0);
        let integer_type = i64_type(span);
        let pointer_type = super::pointer_type(span);
        let mut block = Block::new(
            &context,
            [
                BlockArgument::new(integer_type, span),
                BlockArgument::new(pointer_type, span),
            ],
        );
        let value = block.arguments()[0].into();
        let pointer = block.arguments()[1].into();

        block
            .operations_mut()
            .push_back(store(&context, value, pointer, span));
    }
}
