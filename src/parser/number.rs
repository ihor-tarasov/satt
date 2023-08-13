use super::*;
use crate::combs::{any, Parse};

fn digit(radix: u32) -> impl Parse<u32> {
    any().filter_map(move |c| c.to_digit(radix))
}

fn integer(radix: u32) -> impl Parse<i64> {
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

fn real() -> impl Parse<f64> {
    integer(10)
        .and(sym('.'))
        .and(integer(10))
        .map(|((a, _), b)| {
            let count = (b as f64).log10().floor() as i32 + 1;
            a as f64 + b as f64 / 10f64.powi(count)
        })
}

pub fn number() -> impl Parse<Node> {
    real()
        .map(|value| Node::Real(value))
        .or(integer(10).map(|value| Node::Integer(value)))
}
