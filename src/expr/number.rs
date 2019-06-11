use crate::expr::{Expr, SimpleExpr};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Number {
    UInt(u64),
    Int(i64),
    Double(f64),
    Float(f32),
}

impl Number {
    pub fn is_u64(&self) -> bool {
        match self {
            Number::UInt(_) => true,
            _ => false,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Number::UInt(u) => Some(*u),
            _ => None,
        }
    }

    pub fn is_i64(&self) -> bool {
        match self {
            Number::Int(_) => true,
            _ => false,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Number::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn is_f64(&self) -> bool {
        match self {
            Number::Double(_) => true,
            _ => false,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Number::Double(f) => Some(*f),
            _ => None,
        }
    }

    pub fn is_f32(&self) -> bool {
        match self {
            Number::Float(_) => true,
            _ => false,
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Number::Float(f) => Some(*f),
            _ => None,
        }
    }
}

int_expr!(i8, i16, i32, i64);
uint_expr!(u8, u16, u32, u64);

impl From<f64> for Number {
    fn from(f: f64) -> Number {
        Number::Double(f)
    }
}

impl<'a> From<f32> for Number {
    fn from(f: f32) -> Number {
        Number::Float(f)
    }
}
