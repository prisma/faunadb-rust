pub mod basic;
pub mod collection;
pub mod logical;
pub mod math;
pub mod misc;
pub mod read;
pub mod write;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Query<'a> {
    At(basic::At<'a>),
    Call(basic::Call<'a>),
    Do(basic::Do<'a>),
    Let(basic::Let<'a>),
    Var(basic::Var<'a>),
    Lambda(basic::Lambda<'a>),
    If(basic::If<'a>),

    Append(collection::Append<'a>),
    Drop(collection::Drop<'a>),
    Filter(collection::Filter<'a>),
    Foreach(collection::Foreach<'a>),
    IsEmpty(collection::IsEmpty<'a>),
    IsNonEmpty(collection::IsNonEmpty<'a>),
    Map(collection::Map<'a>),
    Prepend(collection::Prepend<'a>),
    Take(collection::Take<'a>),

    And(logical::And<'a>),
    Or(logical::Or<'a>),
    Not(logical::Not<'a>),
    Lt(logical::Lt<'a>),
    Lte(logical::Lte<'a>),
    Gt(logical::Gt<'a>),
    Gte(logical::Gte<'a>),
    Contains(logical::Contains<'a>),
    Equals(logical::Equals<'a>),
    Exists(logical::Exists<'a>),

    Abs(math::Abs<'a>),
    Acos(math::Acos<'a>),
    Add(math::Add<'a>),
    Asin(math::Asin<'a>),
    Atan(math::Atan<'a>),
    BitAnd(math::BitAnd<'a>),
    BitNot(math::BitNot<'a>),
    BitOr(math::BitOr<'a>),
    BitXor(math::BitXor<'a>),
    Ceil(math::Ceil<'a>),
    Cos(math::Cos<'a>),
    Cosh(math::Cosh<'a>),
    Degrees(math::Degrees<'a>),
    Divide(math::Divide<'a>),
    Exp(math::Exp<'a>),
    Floor(math::Floor<'a>),
    Hypot(math::Hypot<'a>),
    Ln(math::Ln<'a>),
    Log(math::Log<'a>),
    Max(math::Max<'a>),
    Min(math::Min<'a>),
    Modulo(math::Modulo<'a>),
    Multiply(math::Multiply<'a>),
    Pow(math::Pow<'a>),
    Radians(math::Radians<'a>),
    Round(math::Round<'a>),
    Sign(math::Sign<'a>),
    Sin(math::Sin<'a>),
    Sinh(math::Sinh<'a>),
    Sqrt(math::Sqrt<'a>),
    Subtract(math::Subtract<'a>),
    Tan(math::Tan<'a>),
    Tanh(math::Tanh<'a>),
    Trunc(math::Trunc<'a>),

    CreateClass(Box<write::CreateClass<'a>>),
    CreateDatabase(write::CreateDatabase<'a>),
    CreateIndex(Box<write::CreateIndex<'a>>),
    CreateFunction(Box<write::CreateFunction<'a>>),
    Create(write::Create<'a>),
    Delete(write::Delete<'a>),
    Get(read::Get<'a>),

    Classes(misc::Classes<'a>),
    Databases(misc::Databases<'a>),
}
