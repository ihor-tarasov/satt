use satt::{
    parser::{parse, parser},
    Input,
};

mod line_reader;

use line_reader::LineReader;

fn main() {
    let parser = parser();
    let mut reader = LineReader::new();
    loop {
        let line = reader.read();
        let input = Input::new(line, 0);
        match parse(&parser, input) {
            Ok(node) => match node.eval() {
                Ok(value) => println!("{value}"),
                Err(error) => println!("Runime error: {}", error.messgae),
            },
            Err(error) => println!("Compilation error: {}", error.messgae),
        }
    }
}
