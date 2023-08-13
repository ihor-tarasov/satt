use super::*;
use crate::{
    ast::Node,
    combs::{boxed, Parse},
};

pub fn subexpr() -> Box<dyn Parse<Node>> {
    Box::new(
        sym('(')
            .and(whitespaces())
            .and(expression())
            .and(whitespaces())
            .and(expect(sym(')'), || "')'"))
            .map(|((((_, _), n), _), _)| n),
    )
}

pub fn primary() -> impl Parse<Node> {
    expect(number().or(boxed(subexpr)), || "value")
}
