use super::{primary, sym, whitespaces};
use crate::{
    ast::{
        binary::{Binary, BinaryKind, BinaryOperator},
        Node,
    },
    combs::{Parse, Token},
};

fn factor_binary_kind() -> impl Parse<BinaryKind> {
    sym('*')
        .map(|_| BinaryKind::Multiply)
        .or(sym('/').map(|_| BinaryKind::Divide))
        .or(sym('%').map(|_| BinaryKind::Module))
}

fn term_binary_kind() -> impl Parse<BinaryKind> {
    sym('+')
        .map(|_| BinaryKind::Addict)
        .or(sym('-').map(|_| BinaryKind::Subtract))
}

fn shifts_binary_kind() -> impl Parse<BinaryKind> {
    sym('<')
        .and(sym('<'))
        .map(|_| BinaryKind::ShiftLeft)
        .or(sym('>').and(sym('>')).map(|_| BinaryKind::ShiftRight))
}

fn bitwise_and_binary_kind() -> impl Parse<BinaryKind> {
    sym('&').map(|_| BinaryKind::And)
}

fn bitwise_xor_binary_kind() -> impl Parse<BinaryKind> {
    sym('^').map(|_| BinaryKind::Xor)
}

fn bitwise_or_binary_kind() -> impl Parse<BinaryKind> {
    sym('|').map(|_| BinaryKind::Or)
}

fn comparison_binary_kind() -> impl Parse<BinaryKind> {
    sym('=')
        .and(sym('='))
        .map(|_| BinaryKind::Equals)
        .or(sym('!').and(sym('=')).map(|_| BinaryKind::NotEquals))
        .or(sym('<').and(sym('=')).map(|_| BinaryKind::LessEquals))
        .or(sym('>').and(sym('=')).map(|_| BinaryKind::GreaterEquals))
        .or(sym('<').map(|_| BinaryKind::Less))
        .or(sym('>').map(|_| BinaryKind::Greater))
}

fn binary_operator<O>(kind: O) -> impl Parse<BinaryOperator>
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

fn binary_helper<O, N>(kind: O, next: N) -> impl Parse<Node>
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

fn factor() -> impl Parse<Node> {
    binary_helper(factor_binary_kind(), primary())
}

fn term() -> impl Parse<Node> {
    binary_helper(term_binary_kind(), factor())
}

fn shifts() -> impl Parse<Node> {
    binary_helper(shifts_binary_kind(), term())
}

fn bitwise_and() -> impl Parse<Node> {
    binary_helper(bitwise_and_binary_kind(), shifts())
}

fn bitwise_xor() -> impl Parse<Node> {
    binary_helper(bitwise_xor_binary_kind(), bitwise_and())
}

fn bitwise_or() -> impl Parse<Node> {
    binary_helper(bitwise_or_binary_kind(), bitwise_xor())
}

fn comparison() -> impl Parse<Node> {
    binary_helper(comparison_binary_kind(), bitwise_or())
}

pub fn binary() -> impl Parse<Node> {
    comparison()
}
