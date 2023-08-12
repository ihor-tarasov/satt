use super::{Parse, ParseResult};
use crate::Input;

pub struct Filter<A, P, F> {
    pub(super) parse: P,
    pub(super) func: F,
    pub(super) phantom: core::marker::PhantomData<A>,
}

impl<A, P, F> Parse<A> for Filter<A, P, F>
where
    P: Parse<A>,
    F: Fn(&A) -> bool,
{
    fn parse(&self, input: Input) -> ParseResult<A> {
        self.parse.parse(input).and_then(|data| {
            Ok(data.and_then(|data| {
                if (self.func)(&data.token.value) {
                    Some(data)
                } else {
                    None
                }
            }))
        })
    }
}
