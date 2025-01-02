/// An attribute.
#[derive(Clone, Copy, Debug)]
pub enum Attribute<'a> {
    /// An `i32` value.
    I32(i32),
    /// An `i64` value.
    I64(i64),
    /// A `u32` value.
    U32(u32),
    /// A `u64` value.
    U64(u64),
    /// A `string` value.
    String(&'a str),
}
