use super::{Parse, ParseResult};
use crate::Input;

pub struct Or<L, R, A> {
    pub(super) left: L,
    pub(super) right: R,
    pub(super) phantom: core::marker::PhantomData<A>,
}

impl<L, R, A> Parse<A> for Or<L, R, A>
where
    L: Parse<A>,
    R: Parse<A>,
{
    fn parse(&self, input: Input) -> ParseResult<A> {
        match self.left.parse(input.clone())? {
            Some(data) => return Ok(Some(data)),
            None => (),
        }
        match self.right.parse(input.clone())? {
            Some(data) => return Ok(Some(data)),
            None => (),
        }
        Ok(None)
    }
}
