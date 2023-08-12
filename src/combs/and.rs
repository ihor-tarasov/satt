use super::{Parse, ParseResult, ParsedData, Token};
use crate::Input;

pub struct And<L, R, A, B> {
    pub(super) left: L,
    pub(super) right: R,
    pub(super) phantom: core::marker::PhantomData<(A, B)>,
}

impl<L, R, A, B> Parse<(A, B)> for And<L, R, A, B>
where
    L: Parse<A>,
    R: Parse<B>,
{
    fn parse(&self, mut input: Input) -> ParseResult<(A, B)> {
        let mut range: Option<core::ops::Range<usize>> = None;
        let result = (
            match self.left.parse(input)? {
                Some(data) => {
                    input = data.rest;
                    if let Some(range) = range.as_mut() {
                        range.end = data.token.pos.end;
                    } else {
                        range = Some(data.token.pos);
                    }
                    data.token.value
                }
                None => return Ok(None),
            },
            match self.right.parse(input)? {
                Some(data) => {
                    input = data.rest;
                    if let Some(range) = range.as_mut() {
                        range.end = data.token.pos.end;
                    } else {
                        range = Some(data.token.pos);
                    }
                    data.token.value
                }
                None => return Ok(None),
            },
        );
        Ok(Some(ParsedData {
            token: Token {
                value: result,
                pos: range.unwrap(),
                source_id: input.source_id(),
            },
            rest: input,
        }))
    }
}
