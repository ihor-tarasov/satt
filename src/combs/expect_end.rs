use super::{Parse, ParseResult};
use crate::{Error, Input};

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
