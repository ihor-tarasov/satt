use super::{Parse, ParseResult, ParsedData, Token};
use crate::Input;

pub struct OneOrMoreSep<A, B, P, S> {
    pub(super) parse_element: P,
    pub(super) parse_separator: S,
    pub(super) phantom: core::marker::PhantomData<(A, B)>,
}

impl<A, B, P, S> Parse<(A, Vec<(B, A)>)> for OneOrMoreSep<A, B, P, S>
where
    P: Parse<A>,
    S: Parse<B>,
{
    fn parse(&self, mut input: Input) -> ParseResult<(A, Vec<(B, A)>)> {
        let source_id = input.source_id();
        let (start, first) = if let Some(first) = self.parse_element.parse(input.clone())? {
            input = first.rest;
            (first.token.pos.start, first.token.value)
        } else {
            return Ok(None);
        };

        let mut v = Vec::new();
        let mut end = start + 1;

        loop {
            let item = (
                if let Some(separator) = self.parse_separator.parse(input.clone())? {
                    input = separator.rest;
                    end = separator.token.pos.end;
                    separator.token.value
                } else {
                    break;
                },
                if let Some(element) = self.parse_element.parse(input.clone())? {
                    input = element.rest;
                    end = element.token.pos.end;
                    element.token.value
                } else {
                    break;
                },
            );
            v.push(item);
        }

        Ok(Some(ParsedData {
            token: Token {
                value: (first, v),
                pos: start..end,
                source_id,
            },
            rest: input,
        }))
    }
}
