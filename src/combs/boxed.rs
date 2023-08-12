use super::{Parse, ParseResult};
use crate::Input;

pub struct Boxed<F, A> {
    func: F,
    phantom: core::marker::PhantomData<A>,
}

impl<F, A> Parse<A> for Boxed<F, A>
where
    F: Fn() -> Box<dyn Parse<A>>,
{
    fn parse(&self, input: Input) -> ParseResult<A> {
        (self.func)().parse(input)
    }
}

pub fn boxed<F, A>(f: F) -> impl Parse<A>
where
    F: Fn() -> Box<dyn Parse<A>>,
{
    Boxed {
        func: f,
        phantom: std::marker::PhantomData,
    }
}
