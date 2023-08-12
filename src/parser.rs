use crate::{
    ast::{
        binary::{Binary, BinaryKind, BinaryOperator},
        Node,
    },
    combs::{boxed, expect_ext, integer, real, sym, whitespaces, Parse, Token},
};

pub fn number() -> impl Parse<Node> {
    real()
        .map(|value| Node::Real(value))
        .or(integer(10).map(|value| Node::Integer(value)))
}

pub fn subexpr() -> Box<dyn Parse<Node>> {
    Box::new(
        sym('(')
            .and(whitespaces())
            .and(expression())
            .and(whitespaces())
            .and(expect_ext(sym(')'), || "')'"))
            .map(|((((_, _), n), _), _)| n),
    )
}

pub fn primary() -> impl Parse<Node> {
    expect_ext(number().or(boxed(subexpr)), || "value")
}

pub fn factor_binary_kind() -> impl Parse<BinaryKind> {
    sym('*')
        .map(|_| BinaryKind::Multiply)
        .or(sym('/').map(|_| BinaryKind::Divide))
        .or(sym('%').map(|_| BinaryKind::Module))
}

pub fn term_binary_kind() -> impl Parse<BinaryKind> {
    sym('+')
        .map(|_| BinaryKind::Addict)
        .or(sym('-').map(|_| BinaryKind::Subtract))
}

pub fn shifts_binary_kind() -> impl Parse<BinaryKind> {
    sym('<')
        .and(sym('<'))
        .map(|_| BinaryKind::ShiftLeft)
        .or(sym('>').and(sym('>')).map(|_| BinaryKind::ShiftRight))
}

pub fn bitwise_and_binary_kind() -> impl Parse<BinaryKind> {
    sym('&').map(|_| BinaryKind::And)
}

pub fn bitwise_xor_binary_kind() -> impl Parse<BinaryKind> {
    sym('^').map(|_| BinaryKind::Xor)
}

pub fn bitwise_or_binary_kind() -> impl Parse<BinaryKind> {
    sym('|').map(|_| BinaryKind::Or)
}

pub fn comparison_binary_kind() -> impl Parse<BinaryKind> {
    sym('=')
        .and(sym('='))
        .map(|_| BinaryKind::Equals)
        .or(sym('!').and(sym('=')).map(|_| BinaryKind::NotEquals))
        .or(sym('<').and(sym('=')).map(|_| BinaryKind::LessEquals))
        .or(sym('>').and(sym('=')).map(|_| BinaryKind::GreaterEquals))
        .or(sym('<').map(|_| BinaryKind::Less))
        .or(sym('>').map(|_| BinaryKind::Greater))
}

pub fn binary_operator<O>(kind: O) -> impl Parse<BinaryOperator>
where
    O: Parse<BinaryKind>,
{
    whitespaces()
        .and(kind)
        .and(whitespaces())
        .map_token(|token| Token {
            value: BinaryOperator {
                kind: token.value.0 .1,
                pos: token.pos.clone(),
                source_id: token.source_id,
            },
            pos: token.pos,
            source_id: token.source_id,
        })
}

pub fn binary<O, N>(kind: O, next: N) -> impl Parse<Node>
where
    O: Parse<BinaryKind>,
    N: Parse<Node>,
{
    next.one_or_more_sep(binary_operator(kind))
        .map(|(first, others)| {
            if others.is_empty() {
                first
            } else {
                Node::Binary(Box::new(Binary { first, others }))
            }
        })
}

pub fn factor() -> impl Parse<Node> {
    binary(factor_binary_kind(), primary())
}

pub fn term() -> impl Parse<Node> {
    binary(term_binary_kind(), factor())
}

pub fn shifts() -> impl Parse<Node> {
    binary(shifts_binary_kind(), term())
}

pub fn bitwise_and() -> impl Parse<Node> {
    binary(bitwise_and_binary_kind(), shifts())
}

pub fn bitwise_xor() -> impl Parse<Node> {
    binary(bitwise_xor_binary_kind(), bitwise_and())
}

pub fn bitwise_or() -> impl Parse<Node> {
    binary(bitwise_or_binary_kind(), bitwise_xor())
}

pub fn comparison() -> impl Parse<Node> {
    binary(comparison_binary_kind(), bitwise_or())
}

pub fn expression<'a>() -> impl Parse<Node> {
    comparison()
}

pub fn parser<'a>() -> impl Parse<Node> {
    whitespaces()
        .and(expression())
        .and(whitespaces())
        .map(|((_, n), _)| n)
        .map(|node| Node::Root(Box::new(node)))
        .expect_end()
}
