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

pub fn expect_ext<T, P, F>(p: P, f: F) -> impl Parse<T>
where
    P: Parse<T>,
    F: Fn() -> &'static str + 'static,
{
    p.expect(move |mut input| {
        if let Some(c) = input.next() {
            format!("Expected {}, found '{}'.", f(), c)
        } else {
            format!("Expected {}, found end of code.", f())
        }
    })
}

pub struct ExpectEnd<P, A> {
    pub(super) parse: P,
    pub(super) phantom: core::marker::PhantomData<A>,
}

impl<P, A> Parse<A> for ExpectEnd<P, A>
where
    P: Parse<A>,
{
    fn parse(&self, input: Input) -> ParseResult<A> {
        match self.parse.parse(input) {
            Ok(data) => match data {
                Some(mut data) => {
                    let source_id = data.rest.source_id();
                    let offset = data.rest.offset();
                    if let Some(c) = data.rest.next() {
                        Err(Error {
                            messgae: format!("Expected end of code, found '{c}'."),
                            pos: offset..(offset + 1),
                            source_id: source_id,
                        })
                    } else {
                        Ok(Some(data))
                    }
                }
                None => Ok(None),
            },
            Err(error) => Err(error),
        }
    }
}
