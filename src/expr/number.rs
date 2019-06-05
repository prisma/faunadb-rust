use crate::expr::{Expr, SimpleExpr};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Number {
    UInt(u64),
    Int(i64),
    Double(f64),
    Float(f32),
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
