use crate::Input;
use super::{Parse, ParseResult, ParsedData, Token};

pub struct Fold<A, B, P, I, F> {
    pub(super) parse: P,
    pub(super) func: F,
    pub(super) init: I,
    pub(super) phantom: core::marker::PhantomData<(A, B)>,
}

impl<A, B, P, I, F> Parse<B> for Fold<A, B, P, I, F>
where
    P: Parse<A>,
    F: Fn(B, A) -> B,
    I: Fn() -> B,
{
    fn parse(&self, mut input: Input) -> ParseResult<B> {
        let mut t = (self.init)();
        let mut pos: Option<core::ops::Range<usize>> = None;
        loop {
            match self.parse.parse(input.clone())? {
                Some(data) => {
                    t = (self.func)(t, data.token.value);
                    if let Some(pos) = pos.as_mut() {
                        pos.end = data.token.pos.end;
                    } else {
                        pos = Some(data.token.pos);
                    }
                    input = data.rest;
                }
                None => {
                    break Ok(Some(ParsedData {
                        token: Token {
                            value: t,
                            pos: pos.unwrap_or_else(|| input.offset()..(input.offset() + 1)),
                            source_id: input.source_id(),
                        },
                        rest: input,
                    }));
                }
            }
        }
    }
}
