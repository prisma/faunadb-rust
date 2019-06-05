mod basic;
mod collection;
mod logical;
mod math;
mod read;
mod write;

pub use basic::*;
pub use collection::*;
pub use logical::*;
pub use math::*;
pub use read::*;
pub use write::*;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Query<'a> {
    Create(Create<'a>),
    CreateClass(Box<CreateClass<'a>>),
    CreateDatabase(CreateDatabase<'a>),
    CreateIndex(Box<CreateIndex<'a>>),
    Delete(Delete<'a>),
    Get(Get<'a>),
    Do(Do<'a>),
    If(If<'a>),
    Let(Let<'a>),
    Var(Var<'a>),
    Lambda(Lambda<'a>),
    Map(Map<'a>),
    And(And<'a>),
    Or(Or<'a>),
    Not(Not<'a>),
    Lt(Lt<'a>),
    Lte(Lte<'a>),
    Gt(Gt<'a>),
    Gte(Gte<'a>),
    Contains(Contains<'a>),
    Equals(Equals<'a>),
    Exists(Exists<'a>),
    Abs(Abs<'a>),
    Acos(Acos<'a>),
    Add(Add<'a>),
    Asin(Asin<'a>),
    Atan(Atan<'a>),
    BitAnd(BitAnd<'a>),
    BitNot(BitNot<'a>),
    BitOr(BitOr<'a>),
    BitXor(BitXor<'a>),
    Ceil(Ceil<'a>),
    Cos(Cos<'a>),
    Cosh(Cosh<'a>),
    Degrees(Degrees<'a>),
    Divide(Divide<'a>),
    Exp(Exp<'a>),
    Floor(Floor<'a>),
    Hypot(Hypot<'a>),
    Ln(Ln<'a>),
    Log(Log<'a>),
    Max(Max<'a>),
    Min(Min<'a>),
    Modulo(Modulo<'a>),
    Multiply(Multiply<'a>),
    Pow(Pow<'a>),
    Radians(Radians<'a>),
    Round(Round<'a>),
    Sign(Sign<'a>),
    Sin(Sin<'a>),
    Sinh(Sinh<'a>),
    Sqrt(Sqrt<'a>),
    Subtract(Subtract<'a>),
    Tan(Tan<'a>),
    Tanh(Tanh<'a>),
    Trunc(Trunc<'a>),
}
