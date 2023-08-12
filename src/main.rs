use std::{io::Write, rc::Rc};
use tpc::{combs::Parse, parser::parser, Input};

fn main() {
    let parser = parser();
    let mut line = Rc::new(String::new());
    loop {
        Rc::make_mut(&mut line).clear();
        print!("-> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(Rc::make_mut(&mut line)).unwrap();
        let input = Input::new(line.clone(), 0);
        let result = parser.parse(input);
        match result {
            Ok(data) => match data {
                Some(data) => match data.token.value.eval() {
                    Ok(value) => println!("{value}"),
                    Err(error) => println!("Runtime error: {}", error.messgae),
                },
                None => println!("None returned."),
            },
            Err(error) => println!("Compile error: {}", error.messgae),
        }
    }
}
