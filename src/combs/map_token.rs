use super::{Parse, ParseResult, ParsedData, Token};
use crate::Input;

pub struct MapToken<A, B, P, F> {
    pub(super) parse: P,
    pub(super) func: F,
    pub(super) phantom: core::marker::PhantomData<(A, B)>,
}

impl<A, B, P, F> Parse<B> for MapToken<A, B, P, F>
where
    P: Parse<A>,
    F: Fn(Token<A>) -> Token<B>,
{
    fn parse(&self, input: Input) -> ParseResult<B> {
        self.parse.parse(input).and_then(|data| {
            Ok(data.and_then(|data| {
                Some(ParsedData {
                    token: (self.func)(data.token),
                    rest: data.rest,
                })
            }))
        })
    }
}
