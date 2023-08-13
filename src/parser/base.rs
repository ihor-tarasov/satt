use crate::combs::{any, Parse};

pub fn sym(c: char) -> impl Parse<char> {
    any().filter(move |oc| c == *oc)
}

pub fn whitespaces() -> impl Parse<()> {
    any()
        .filter_map(|c| if c.is_whitespace() { Some(()) } else { None })
        .fold(|| (), |_, _| ())
}

pub fn expect<T, P, F>(p: P, f: F) -> impl Parse<T>
where
    P: Parse<T>,
    F: Fn() -> &'static str + 'static,
{
    p.expect(move |mut input| {
        if let Some(c) = input.next() {
            format!("Expected {}, found '{}'.", f(), c)
        } else {
            format!("Expected {}, found end of code.", f())
        }
    })
}
