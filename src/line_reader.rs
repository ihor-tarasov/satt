use std::{io::Write, rc::Rc};

pub struct LineReader(Rc<String>);

impl LineReader {
    pub fn new() -> Self {
        Self(Rc::new(String::new()))
    }

    pub fn read(&mut self) -> Rc<String> {
        Rc::make_mut(&mut self.0).clear();
        print!("-> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(Rc::make_mut(&mut self.0))
            .unwrap();
        Rc::clone(&self.0)
    }
}
