#[derive(Debug, PartialEq)]
pub struct Error {
    pub messgae: String,
    pub pos: core::ops::Range<usize>,
    pub source_id: usize,
}
