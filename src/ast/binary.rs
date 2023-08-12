use crate::{Error, Value};

use super::Node;

#[derive(Debug, Clone, Copy)]
pub enum BinaryKind {
    Addict,
    Subtract,
    Multiply,
    Divide,
    Module,
    And,
    Or,
    Xor,
    ShiftLeft,
    ShiftRight,
    Equals,
    NotEquals,
    Less,
    Greater,
    LessEquals,
    GreaterEquals,
}

#[derive(Debug)]
pub struct BinaryOperator {
    pub kind: BinaryKind,
    pub pos: core::ops::Range<usize>,
    pub source_id: usize,
}

#[derive(Debug)]
pub struct Binary {
    pub first: Node,
    pub others: Vec<(BinaryOperator, Node)>,
}

impl Binary {
    fn eval_pair(kind: BinaryKind, l: Value, r: Value) -> Result<Value, String> {
        match kind {
            BinaryKind::Addict => l + r,
            BinaryKind::Multiply => l * r,
            BinaryKind::Subtract => l - r,
            BinaryKind::Divide => l / r,
            BinaryKind::Module => l % r,
            BinaryKind::And => l & r,
            BinaryKind::Or => l | r,
            BinaryKind::Xor => l ^ r,
            BinaryKind::ShiftLeft => l << r,
            BinaryKind::ShiftRight => l >> r,
            BinaryKind::Equals => Ok(Value::Boolean(l == r)),
            BinaryKind::NotEquals => Ok(Value::Boolean(l != r)),
            BinaryKind::Less => Ok(Value::Boolean(l < r)),
            BinaryKind::Greater => Ok(Value::Boolean(l > r)),
            BinaryKind::LessEquals => Ok(Value::Boolean(l <= r)),
            BinaryKind::GreaterEquals => Ok(Value::Boolean(l >= r)),
        }
    }

    pub fn eval(&self) -> Result<Value, Error> {
        let mut result = self.first.eval()?;

        for (operator, node) in self.others.iter() {
            let right = node.eval()?;
            result = Self::eval_pair(operator.kind, result, right).map_err(|m| Error {
                messgae: m,
                pos: operator.pos.clone(),
                source_id: operator.source_id,
            })?;
        }

        Ok(result)
    }
}
