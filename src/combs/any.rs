use super::{Parse, ParseResult, ParsedData, Token};
use crate::Input;

pub struct Any;

impl Parse<char> for Any {
    fn parse(&self, mut input: Input) -> ParseResult<char> {
        let source_id = input.source_id();
        let offset = input.offset();
        Ok(input.next().map(|c| ParsedData {
            token: Token {
                value: c,
                pos: offset..(offset + 1),
                source_id,
            },
            rest: input,
        }))
    }
}

pub fn any() -> impl Parse<char> {
    Any
}
