use super::{Parse, ParseResult, ParsedData, Token};
use crate::Input;

pub struct FilterMap<A, B, P, F> {
    pub(super) parse: P,
    pub(super) func: F,
    pub(super) phantom: core::marker::PhantomData<(A, B)>,
}

impl<A, B, P, F> Parse<B> for FilterMap<A, B, P, F>
where
    P: Parse<A>,
    F: Fn(A) -> Option<B>,
{
    fn parse(&self, input: Input) -> ParseResult<B> {
        self.parse.parse(input).and_then(|data| {
            Ok(data.and_then(|data| {
                Some(ParsedData {
                    token: Token {
                        value: (self.func)(data.token.value)?,
                        pos: data.token.pos,
                        source_id: data.token.source_id,
                    },
                    rest: data.rest,
                })
            }))
        })
    }
}
