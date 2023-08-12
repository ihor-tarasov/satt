use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Value {
    Void,
    Boolean(bool),
    Integer(i64),
    Real(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Void => write!(f, "()"),
            Value::Boolean(value) => write!(f, "{value}"),
            Value::Integer(value) => write!(f, "{value}"),
            Value::Real(value) => write!(f, "{value}"),
        }
    }
}

impl std::ops::Add for Value {
    type Output = Result<Self, String>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l.wrapping_add(r))),
            (Value::Integer(l), Value::Real(r)) => Ok(Value::Real(l as f64 + r)),
            (Value::Real(l), Value::Integer(r)) => Ok(Value::Real(l + r as f64)),
            (Value::Real(l), Value::Real(r)) => Ok(Value::Real(l + r)),
            _ => Err(format!(
                "Operator '+' is unsupported for {self} and {rhs} values."
            )),
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Result<Self, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l.wrapping_sub(r))),
            (Value::Integer(l), Value::Real(r)) => Ok(Value::Real(l as f64 - r)),
            (Value::Real(l), Value::Integer(r)) => Ok(Value::Real(l - r as f64)),
            (Value::Real(l), Value::Real(r)) => Ok(Value::Real(l - r)),
            _ => Err(format!(
                "Operator '-' is unsupported for {self} and {rhs} values."
            )),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Result<Self, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l.wrapping_mul(r))),
            (Value::Integer(l), Value::Real(r)) => Ok(Value::Real(l as f64 * r)),
            (Value::Real(l), Value::Integer(r)) => Ok(Value::Real(l * r as f64)),
            (Value::Real(l), Value::Real(r)) => Ok(Value::Real(l * r)),
            _ => Err(format!(
                "Operator '*' is unsupported for {self} and {rhs} values."
            )),
        }
    }
}

impl std::ops::Div for Value {
    type Output = Result<Self, String>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(l), Value::Integer(r)) => {
                if r == 0 {
                    Err(format!("Dividing by zero."))
                } else {
                    Ok(Value::Integer(l.wrapping_div(r)))
                }
            }
            (Value::Integer(l), Value::Real(r)) => Ok(Value::Real(l as f64 / r)),
            (Value::Real(l), Value::Integer(r)) => Ok(Value::Real(l / r as f64)),
            (Value::Real(l), Value::Real(r)) => Ok(Value::Real(l / r)),
            _ => Err(format!(
                "Operator '/' is unsupported for {self} and {rhs} values."
            )),
        }
    }
}

impl std::ops::Rem for Value {
    type Output = Result<Self, String>;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(l), Value::Integer(r)) => {
                if r == 0 {
                    Err(format!("Dividing by zero."))
                } else {
                    Ok(Value::Integer(l.wrapping_rem(r)))
                }
            }
            (Value::Integer(l), Value::Real(r)) => Ok(Value::Real(l as f64 % r)),
            (Value::Real(l), Value::Integer(r)) => Ok(Value::Real(l % r as f64)),
            (Value::Real(l), Value::Real(r)) => Ok(Value::Real(l % r)),
            _ => Err(format!(
                "Operator '%' is unsupported for {self} and {rhs} values."
            )),
        }
    }
}

impl std::ops::Shl for Value {
    type Output = Result<Self, String>;

    fn shl(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(l), Value::Integer(r)) => {
                if r < 0 {
                    Err(format!(
                        "Operator '<<' is unsupported for negative right hand side value {r}."
                    ))
                } else if r > u32::MAX as i64 {
                    Err(format!(
                        "Operator '<<' is unsupported for big right hand side value {r}."
                    ))
                } else {
                    Ok(Value::Integer(l.wrapping_shl(r as u32)))
                }
            }
            _ => Err(format!(
                "Operator '<<' is unsupported for {self} and {rhs} values."
            )),
        }
    }
}

impl std::ops::Shr for Value {
    type Output = Result<Self, String>;

    fn shr(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(l), Value::Integer(r)) => {
                if r < 0 {
                    Err(format!(
                        "Operator '>>' is unsupported for negative right hand side value {r}."
                    ))
                } else if r > u32::MAX as i64 {
                    Err(format!(
                        "Operator '>>' is unsupported for big right hand side value {r}."
                    ))
                } else {
                    Ok(Value::Integer(l.wrapping_shr(r as u32)))
                }
            }
            _ => Err(format!(
                "Operator '>>' is unsupported for {self} and {rhs} values."
            )),
        }
    }
}

impl std::ops::BitAnd for Value {
    type Output = Result<Self, String>;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l & r)),
            _ => Err(format!(
                "Operator '&' is unsupported for {self} and {rhs} values."
            )),
        }
    }
}

impl std::ops::BitOr for Value {
    type Output = Result<Self, String>;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l | r)),
            _ => Err(format!(
                "Operator '|' is unsupported for {self} and {rhs} values."
            )),
        }
    }
}

impl std::ops::BitXor for Value {
    type Output = Result<Self, String>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l ^ r)),
            _ => Err(format!(
                "Operator '^' is unsupported for {self} and {rhs} values."
            )),
        }
    }
}
