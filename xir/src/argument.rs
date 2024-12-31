use crate::Type;

pub struct Argument<T: Type> {
    r#type: T,
}

impl<T: Type> Argument<T> {
    pub fn new(r#type: T) -> Self {
        Self { r#type }
    }

    pub fn r#type(&self) -> &T {
        &self.r#type
    }
}
