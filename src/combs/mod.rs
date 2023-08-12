mod and;
mod any;
mod boxed;
mod expect;
mod filter;
mod filter_map;
mod fold;
mod list;
mod map;
mod map_token;
mod or;

pub use and::*;
pub use any::*;
pub use boxed::*;
pub use expect::*;
pub use filter::*;
pub use filter_map::*;
pub use fold::*;
pub use list::*;
pub use map::*;
pub use map_token::*;
pub use or::*;

use crate::{Input, Error};

#[derive(Debug, PartialEq)]
pub struct Token<T> {
    pub value: T,
    pub pos: core::ops::Range<usize>,
    pub source_id: usize,
}

#[derive(Debug)]
pub struct ParsedData<T> {
    pub token: Token<T>,
    pub rest: Input,
}

pub type ParseResult<T> = Result<Option<ParsedData<T>>, Error>;

pub trait Parse<T> {
    fn parse(&self, input: Input) -> ParseResult<T>;

    fn filter<F>(self, f: F) -> Filter<T, Self, F>
    where
        F: Fn(&T) -> bool,
        Self: Sized,
    {
        Filter {
            parse: self,
            func: f,
            phantom: core::marker::PhantomData,
        }
    }

    fn map<E, F>(self, f: F) -> Map<T, E, Self, F>
    where
        F: Fn(T) -> E,
        Self: Sized,
    {
        Map {
            parse: self,
            func: f,
            phantom: core::marker::PhantomData,
        }
    }

    fn map_token<E, F>(self, f: F) -> MapToken<T, E, Self, F>
    where
        F: Fn(Token<T>) -> Token<E>,
        Self: Sized,
    {
        MapToken {
            parse: self,
            func: f,
            phantom: core::marker::PhantomData,
        }
    }

    fn filter_map<E, F>(self, f: F) -> FilterMap<T, E, Self, F>
    where
        F: Fn(T) -> Option<E>,
        Self: Sized,
    {
        FilterMap {
            parse: self,
            func: f,
            phantom: core::marker::PhantomData,
        }
    }

    fn fold<E, I, F>(self, i: I, f: F) -> Fold<T, E, Self, I, F>
    where
        F: Fn(E, T) -> E,
        I: Fn() -> E,
        Self: Sized,
    {
        Fold {
            parse: self,
            func: f,
            init: i,
            phantom: core::marker::PhantomData,
        }
    }

    fn or<R>(self, r: R) -> Or<Self, R, T>
    where
        Self: Sized,
        R: Parse<T>,
    {
        Or {
            left: self,
            right: r,
            phantom: core::marker::PhantomData,
        }
    }

    fn and<R, E>(self, r: R) -> And<Self, R, T, E>
    where
        Self: Sized,
        R: Parse<E>,
    {
        And {
            left: self,
            right: r,
            phantom: core::marker::PhantomData,
        }
    }

    fn expect<F>(self, f: F) -> Expect<Self, T, F>
    where
        Self: Sized,
        F: Fn(Input) -> String,
    {
        Expect {
            parse: self,
            func: f,
            phantom: core::marker::PhantomData,
        }
    }

    fn expect_end(self) -> ExpectEnd<Self, T>
    where
        Self: Sized,
    {
        ExpectEnd {
            parse: self,
            phantom: core::marker::PhantomData,
        }
    }

    fn one_or_more_sep<E, S>(self, s: S) -> OneOrMoreSep<T, E, Self, S>
    where
        S: Parse<E>,
        Self: Sized,
    {
        OneOrMoreSep {
            parse_element: self,
            parse_separator: s,
            phantom: core::marker::PhantomData,
        }
    }
}

pub fn digit(radix: u32) -> impl Parse<u32> {
    any().filter_map(move |c| c.to_digit(radix))
}

pub fn integer(radix: u32) -> impl Parse<i64> {
    digit(radix)
        .fold(
            || None,
            |i, p| {
                if let Some(i) = i {
                    Some(i * 10 + p as i64)
                } else {
                    Some(p as i64)
                }
            },
        )
        .filter_map(|o| o)
}

pub fn sym(c: char) -> impl Parse<char> {
    any().filter(move |oc| c == *oc)
}

pub fn real() -> impl Parse<f64> {
    integer(10)
        .and(sym('.'))
        .and(integer(10))
        .map(|((a, _), b)| {
            let count = (b as f64).log10().floor() as i32 + 1;
            a as f64 + b as f64 / 10f64.powi(count)
        })
}

pub fn whitespaces() -> impl Parse<()> {
    any()
        .filter_map(|c| if c.is_whitespace() { Some(()) } else { None })
        .fold(|| (), |_, _| ())
}
