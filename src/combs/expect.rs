use super::{Parse, ParseResult};
use crate::{Error, Input};

pub struct Expect<P, A, F> {
    pub(super) parse: P,
    pub(super) func: F,
    pub(super) phantom: core::marker::PhantomData<A>,
}

impl<P, A, F> Parse<A> for Expect<P, A, F>
where
    P: Parse<A>,
    F: Fn(Input) -> String,
{
    fn parse(&self, input: Input) -> ParseResult<A> {
        match self.parse.parse(input.clone()) {
            Ok(data) => match data {
                Some(data) => Ok(Some(data)),
                None => Err(Error {
                    source_id: input.source_id(),
                    pos: input.offset()..(input.offset() + 1),
                    messgae: (self.func)(input),
                }),
            },
            Err(error) => Err(error),
        }
    }
}
