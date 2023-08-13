mod base;
mod binary;
mod number;
mod primary;

use base::*;
use binary::*;
use number::*;
use primary::*;

use crate::{ast::Node, combs::Parse, Error, Input};

fn expression<'a>() -> impl Parse<Node> {
    binary()
}

pub fn parser() -> impl Parse<Node> {
    whitespaces()
        .and(expression())
        .and(whitespaces())
        .map(|((_, n), _)| n)
        .map(|node| Node::Root(Box::new(node)))
        .expect_end()
}

pub fn parse<P>(p: &P, input: Input) -> Result<Node, Error>
where
    P: Parse<Node>,
{
    match p.parse(input)? {
        Some(data) => Ok(data.token.value),
        None => panic!("Worng parser setup."),
    }
}
