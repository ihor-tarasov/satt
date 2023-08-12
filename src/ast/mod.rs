use crate::{Error, Value};

pub mod binary;

#[derive(Debug)]
pub enum Node {
    Root(Box<Node>),
    Integer(i64),
    Real(f64),
    Binary(Box<binary::Binary>),
}

impl Node {
    pub fn eval(&self) -> Result<Value, Error> {
        match self {
            Node::Root(root) => root.eval(),
            Node::Integer(value) => Ok(Value::Integer(*value)),
            Node::Real(value) => Ok(Value::Real(*value)),
            Node::Binary(binary) => binary.eval(),
        }
    }
}
