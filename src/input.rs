use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Input {
    chars: std::rc::Rc<String>,
    offset: usize,
    source_id: usize,
}

impl Input {
    pub fn new(s: Rc<String>, source_id: usize) -> Self {
        Self {
            chars: s,
            offset: 0,
            source_id,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        self.chars.chars().nth(self.offset).and_then(|c| {
            self.offset += 1;
            Some(c)
        })
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn source_id(&self) -> usize {
        self.source_id
    }
}
