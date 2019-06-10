//! Math functions
use crate::{expr::Expr, query::Query};

// Implements From<fun> for Query
query![
    Abs, Acos, Add, Asin, Atan, BitAnd, BitNot, BitOr, BitXor, Ceil, Cos, Cosh, Degrees, Divide,
    Exp, Floor, Hypot, Ln, Log, Max, Min, Modulo, Multiply, Pow, Radians, Round, Sign, Sin, Sinh,
    Sqrt, Subtract, Tan, Tanh, Trunc
];

/// The `Abs` function is used to get the absolute value of a number.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/abs)
#[derive(Debug, Serialize, Clone)]
pub struct Abs<'a> {
    abs: Expr<'a>,
}

impl<'a> Abs<'a> {
    pub fn new(abs: impl Into<Expr<'a>>) -> Self {
        Self { abs: abs.into() }
    }
}

/// The `Acos` function is a trigonometric function which calculates ratios of the
/// lengths of the sides of right triangles. `Acos` returns the arc cosine of a
/// number.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/acos)
#[derive(Debug, Serialize, Clone)]
pub struct Acos<'a> {
    acos: Expr<'a>,
}

impl<'a> Acos<'a> {
    pub fn new(acos: impl Into<Expr<'a>>) -> Self {
        Self { acos: acos.into() }
    }
}

/// The Add function returns the sum of its numeric arguments.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/add)
#[derive(Debug, Serialize, Clone)]
pub struct Add<'a> {
    add: Expr<'a>,
}

impl<'a> Add<'a> {
    pub fn new(add: impl Into<Expr<'a>>) -> Self {
        Self { add: add.into() }
    }
}

/// The `Asin` function is a trigonometric function which calculates ratios of
/// the lengths of the sides of right triangles. `Asin` returns the arc sine of
/// a number.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/asin)
#[derive(Debug, Serialize, Clone)]
pub struct Asin<'a> {
    asin: Expr<'a>,
}

impl<'a> Asin<'a> {
    pub fn new(asin: impl Into<Expr<'a>>) -> Self {
        Self { asin: asin.into() }
    }
}

/// The `Atan` function is a trigonometric function which calculates ratios of
/// the lengths of the sides of right triangles. `Atan` returns the arc tangent
/// of a number.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/atan)
#[derive(Debug, Serialize, Clone)]
pub struct Atan<'a> {
    atan: Expr<'a>,
}

impl<'a> Atan<'a> {
    pub fn new(atan: impl Into<Expr<'a>>) -> Self {
        Self { atan: atan.into() }
    }
}

/// The `BitAnd` function returns the bit to the result if the bit exists in all
/// numbers. The arguments must be numbers, and fractional values are truncated
/// before the operation is applied. The result is the bitwise AND of all the
/// arguments.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/bitand)
#[derive(Debug, Serialize, Clone)]
pub struct BitAnd<'a> {
    bitand: Expr<'a>,
}

impl<'a> BitAnd<'a> {
    pub fn new(bitand: impl Into<Expr<'a>>) -> Self {
        Self {
            bitand: bitand.into(),
        }
    }
}

/// The `BitNot` function returns the Two’s Complement of a number. The argument
/// must be a number, and fractional values are truncated before the operation
/// is applied.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/bitnot)
#[derive(Debug, Serialize, Clone)]
pub struct BitNot<'a> {
    bitnot: Expr<'a>,
}

impl<'a> BitNot<'a> {
    pub fn new(bitnot: impl Into<Expr<'a>>) -> Self {
        Self {
            bitnot: bitnot.into(),
        }
    }
}

/// The `BitOr` function returns the bit to the result if the bit exists in any
/// argument. The arguments must be numbers, and the fractional portion is
/// truncated before the or operation is applied. The result is the bitwise OR
/// of all the arguments.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/bitor)
#[derive(Debug, Serialize, Clone)]
pub struct BitOr<'a> {
    bitor: Expr<'a>,
}

impl<'a> BitOr<'a> {
    pub fn new(bitor: impl Into<Expr<'a>>) -> Self {
        Self {
            bitor: bitor.into(),
        }
    }
}

/// The `BitXor` function returns the bit to the result if the bit exists in only
/// one argument. The arguments must be numbers, and the fractional portion is
/// truncated before the XOR operation is applied. The result is the bitwise
/// exclusive OR of all of the arguments.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/bitxor)
#[derive(Debug, Serialize, Clone)]
pub struct BitXor<'a> {
    bitxor: Expr<'a>,
}

impl<'a> BitXor<'a> {
    pub fn new(bitxor: impl Into<Expr<'a>>) -> Self {
        Self {
            bitxor: bitxor.into(),
        }
    }
}

/// The `Ceil` function returns a value that is greater than or equal to the
/// argument and is equal to the nearest mathematical integer.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/ceil)
#[derive(Debug, Serialize, Clone)]
pub struct Ceil<'a> {
    ceil: Expr<'a>,
}

impl<'a> Ceil<'a> {
    pub fn new(ceil: impl Into<Expr<'a>>) -> Self {
        Self { ceil: ceil.into() }
    }
}

/// The `Cos` function is a trigonometric function which calculates ratios of the
/// lengths of the sides of right triangles. The `Cos` returns the cosine of a
/// number.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/cos)
#[derive(Debug, Serialize, Clone)]
pub struct Cos<'a> {
    cos: Expr<'a>,
}

impl<'a> Cos<'a> {
    pub fn new(cos: impl Into<Expr<'a>>) -> Self {
        Self { cos: cos.into() }
    }
}

/// The `Cosh` function is a trigonometric function which calculates ratios of the
/// lengths of the sides of right triangles. The Cosh returns the hyperbolic
/// cosine of a number.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/cosh)
#[derive(Debug, Serialize, Clone)]
pub struct Cosh<'a> {
    cosh: Expr<'a>,
}

impl<'a> Cosh<'a> {
    pub fn new(cosh: impl Into<Expr<'a>>) -> Self {
        Self { cosh: cosh.into() }
    }
}

/// The `Degrees` function converts a specified number from radians to degrees.
/// The argument is an angle measured in radians, which is converted to an
/// approximate angle measured in degrees.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/degrees)
#[derive(Debug, Serialize, Clone)]
pub struct Degrees<'a> {
    degrees: Expr<'a>,
}

impl<'a> Degrees<'a> {
    pub fn new(degrees: impl Into<Expr<'a>>) -> Self {
        Self {
            degrees: degrees.into(),
        }
    }
}

/// The Divide function computes the quotient of two or more numbers.
///
/// Attempting to divide an empty list results in an "invalid argument" error.
///
/// Attempting to divide any value by zero results in a "invalid argument"
/// error.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/divide)
#[derive(Debug, Serialize, Clone)]
pub struct Divide<'a> {
    divide: Expr<'a>,
}

impl<'a> Divide<'a> {
    pub fn new(divide: impl Into<Expr<'a>>) -> Self {
        Self {
            divide: divide.into(),
        }
    }
}

/// The `Exp` function returns Euler’s number e (approximately 2.71828) raised to
/// a power provided as the operand.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/exp)
#[derive(Debug, Serialize, Clone)]
pub struct Exp<'a> {
    exp: Expr<'a>,
}

impl<'a> Exp<'a> {
    pub fn new(exp: impl Into<Expr<'a>>) -> Self {
        Self { exp: exp.into() }
    }
}

/// The `Floor` function returns the largest value that is less than or equal to
/// the argument and is equal to a mathematical integer.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/floor)
#[derive(Debug, Serialize, Clone)]
pub struct Floor<'a> {
    floor: Expr<'a>,
}

impl<'a> Floor<'a> {
    pub fn new(floor: impl Into<Expr<'a>>) -> Self {
        Self {
            floor: floor.into(),
        }
    }
}

/// The `Hypot` function calculates the length of the hypotenuse of a right-angle
/// triangle given the length of the other two sides.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/hypot)
#[derive(Debug, Serialize, Clone)]
pub struct Hypot<'a> {
    hypot: Expr<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    b: Option<Expr<'a>>,
}

impl<'a> Hypot<'a> {
    pub fn new(a: impl Into<Expr<'a>>, b: impl Into<Expr<'a>>) -> Self {
        Self {
            hypot: a.into(),
            b: Some(b.into()),
        }
    }

    /// The operation assumes an isosceles right triangle where `b` is equal to `a`.
    pub fn isosceles(a: impl Into<Expr<'a>>) -> Self {
        Self {
            hypot: a.into(),
            b: None,
        }
    }
}

/// The `Ln` function returns the natural logarithm (base e) of the operand.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/ln)
#[derive(Debug, Serialize, Clone)]
pub struct Ln<'a> {
    ln: Expr<'a>,
}

impl<'a> Ln<'a> {
    pub fn new(ln: impl Into<Expr<'a>>) -> Self {
        Self { ln: ln.into() }
    }
}

/// The `Log` function returns the natural logarithm (base 10) of the operand.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/log)
#[derive(Debug, Serialize, Clone)]
pub struct Log<'a> {
    log: Expr<'a>,
}

impl<'a> Log<'a> {
    pub fn new(log: impl Into<Expr<'a>>) -> Self {
        Self { log: log.into() }
    }
}

/// The `Max` function returns the largest value in a list of numbers.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/max)
#[derive(Debug, Serialize, Clone)]
pub struct Max<'a> {
    max: Expr<'a>,
}

impl<'a> Max<'a> {
    pub fn new(max: impl Into<Expr<'a>>) -> Self {
        Self { max: max.into() }
    }
}

/// The Min function returns the smallest value in a list of numbers.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/min)
#[derive(Debug, Serialize, Clone)]
pub struct Min<'a> {
    min: Expr<'a>,
}

impl<'a> Min<'a> {
    pub fn new(min: impl Into<Expr<'a>>) -> Self {
        Self { min: min.into() }
    }
}

/// The `Modulo` function computes the remainder after division on a list of
/// numbers. Providing a single operand returns the operand.
///
/// Attempting to call modulo with an empty array results in the error
/// "Non-empty array expected." Check to make sure at least one argument is
/// passed to modulo function.
///
/// Attempting to compute the remainder of a division by zero results in the
/// error "Illegal division by zero."
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/modulo)
#[derive(Debug, Serialize, Clone)]
pub struct Modulo<'a> {
    modulo: Expr<'a>,
}

impl<'a> Modulo<'a> {
    pub fn new(modulo: impl Into<Expr<'a>>) -> Self {
        Self {
            modulo: modulo.into(),
        }
    }
}

/// The `Multiply` function computes the product of a list of numbers. Providing
/// a single number returns the number.
///
/// Attempting to call multiply without any arguments results in the error
/// "Non-empty array expected." Check to make sure at least one argument is
/// passed to the multiply function.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/multiply)
#[derive(Debug, Serialize, Clone)]
pub struct Multiply<'a> {
    multiply: Expr<'a>,
}

impl<'a> Multiply<'a> {
    pub fn new(modulo: impl Into<Expr<'a>>) -> Self {
        Self {
            multiply: modulo.into(),
        }
    }
}

/// The `Pow` function raises its first numeric argument, the base, to the power
/// of its second numeric argument, the exponent.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/pow)
#[derive(Debug, Serialize, Clone)]
pub struct Pow<'a> {
    pow: Expr<'a>,
    exp: Expr<'a>,
}

impl<'a> Pow<'a> {
    pub fn new(pow: impl Into<Expr<'a>>, exp: impl Into<Expr<'a>>) -> Self {
        Self {
            pow: pow.into(),
            exp: exp.into(),
        }
    }
}

/// The `Radians` function translates a specified number from degrees to
/// radians.
///
/// The argument is an angle measured in degrees, which is converted to
/// the approximated angle measured in radians.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/radians)
#[derive(Debug, Serialize, Clone)]
pub struct Radians<'a> {
    radians: Expr<'a>,
}

impl<'a> Radians<'a> {
    pub fn new(radians: impl Into<Expr<'a>>) -> Self {
        Self {
            radians: radians.into(),
        }
    }
}

/// The `Round` function returns a number which is the nearest mathematic value of
/// the operand to the specified precision as a double value.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/round)
#[derive(Debug, Serialize, Clone)]
pub struct Round<'a> {
    round: Expr<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    precision: Option<Expr<'a>>,
}

impl<'a> Round<'a> {
    pub fn new(round: impl Into<Expr<'a>>) -> Self {
        Self {
            round: round.into(),
            precision: None,
        }
    }

    /// Defines how many digits to the right or left of the decimal place should
    /// be returned. The default precision is 2 which returns up to the
    /// hundredths decimal places. A positive precision specifies digits to the
    /// right of the decimal point. A negative precision specifies digits to the
    /// left of the decimal point. A zero precision rounds the fractional part
    /// of the number.
    pub fn precision(&mut self, precision: impl Into<Expr<'a>>) -> &mut Self {
        self.precision = Some(precision.into());
        self
    }
}

/// The `Sign` function returns the sign of the argument as a numeric value.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/sign)
#[derive(Debug, Serialize, Clone)]
pub struct Sign<'a> {
    sign: Expr<'a>,
}

impl<'a> Sign<'a> {
    pub fn new(sign: impl Into<Expr<'a>>) -> Self {
        Self { sign: sign.into() }
    }
}

/// The `Sin` function is a trigonometric function which calculates ratios of
/// the lengths of the sides of right triangles. The `Sin` returns the sine of a
/// number, which is the ratio of the side opposite to an acute angle in a right
/// triangle to the hypotenuse.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/sin)
#[derive(Debug, Serialize, Clone)]
pub struct Sin<'a> {
    sin: Expr<'a>,
}

impl<'a> Sin<'a> {
    pub fn new(sin: impl Into<Expr<'a>>) -> Self {
        Self { sin: sin.into() }
    }
}

/// A trigonometric function that returns the hyperbolic sine of a number.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/sinh)
#[derive(Debug, Serialize, Clone)]
pub struct Sinh<'a> {
    sinh: Expr<'a>,
}

impl<'a> Sinh<'a> {
    pub fn new(sinh: impl Into<Expr<'a>>) -> Self {
        Self { sinh: sinh.into() }
    }
}

/// The `Sqrt` function returns the positive square root of a number value.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/sqrt)
#[derive(Debug, Serialize, Clone)]
pub struct Sqrt<'a> {
    sqrt: Expr<'a>,
}

impl<'a> Sqrt<'a> {
    pub fn new(sqrt: impl Into<Expr<'a>>) -> Self {
        Self { sqrt: sqrt.into() }
    }
}

/// The `Subtract` function returns the difference of a list of numbers.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/subtract)
#[derive(Debug, Serialize, Clone)]
pub struct Subtract<'a> {
    subtract: Expr<'a>,
}

impl<'a> Subtract<'a> {
    pub fn new(subtract: impl Into<Expr<'a>>) -> Self {
        Self {
            subtract: subtract.into(),
        }
    }
}

/// A trigonometric function that returns the tangent of a number, which is the
/// ratio of the side opposite to an acute angle in a right triangle to the side
/// adjacent.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/tan)
#[derive(Debug, Serialize, Clone)]
pub struct Tan<'a> {
    tan: Expr<'a>,
}

impl<'a> Tan<'a> {
    pub fn new(tan: impl Into<Expr<'a>>) -> Self {
        Self { tan: tan.into() }
    }
}

/// A trigonometric function that returns the hyperbolic tangent of a number.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/tanh)
#[derive(Debug, Serialize, Clone)]
pub struct Tanh<'a> {
    tanh: Expr<'a>,
}

impl<'a> Tanh<'a> {
    pub fn new(tanh: impl Into<Expr<'a>>) -> Self {
        Self { tanh: tanh.into() }
    }
}

/// The `Trunc` function returns a number which is the nearest mathematic value
/// less than or equal to the operand to the specified precision as a double
/// value.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/math/round)
#[derive(Debug, Serialize, Clone)]
pub struct Trunc<'a> {
    trunc: Expr<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    precision: Option<Expr<'a>>,
}

impl<'a> Trunc<'a> {
    pub fn new(trunc: impl Into<Expr<'a>>) -> Self {
        Self {
            trunc: trunc.into(),
            precision: None,
        }
    }

    /// Defines how many digits to the right or left of the decimal place should
    /// be returned. The default precision is 2 which returns up to the
    /// hundredths decimal places. A positive precision specifies digits to the
    /// right of the decimal point. A negative precision specifies digits to the
    /// left of the decimal point. A zero precision truncates the fractional
    /// part of the number.
    pub fn precision(&mut self, precision: impl Into<Expr<'a>>) -> &mut Self {
        self.precision = Some(precision.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde_json::{self, json};

    #[test]
    fn test_abs() {
        let abs = Abs::new(-1);
        let query = Query::from(abs);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"abs": -1}), serialized);
    }

    #[test]
    fn test_acos() {
        let acos = Acos::new(0.63123);
        let query = Query::from(acos);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"acos": 0.63123}), serialized);
    }

    #[test]
    fn test_asin() {
        let fun = Asin::new(0.63123);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"asin": 0.63123}), serialized);
    }

    #[test]
    fn test_atan() {
        let fun = Atan::new(0.63123);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"atan": 0.63123}), serialized);
    }

    #[test]
    fn test_tan() {
        let fun = Tan::new(0.63123);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"tan": 0.63123}), serialized);
    }

    #[test]
    fn test_tanh() {
        let fun = Tanh::new(0.63123);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"tanh": 0.63123}), serialized);
    }

    #[test]
    fn test_cos() {
        let fun = Cos::new(0.63123);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"cos": 0.63123}), serialized);
    }

    #[test]
    fn test_sin() {
        let fun = Sin::new(0.63123);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"sin": 0.63123}), serialized);
    }

    #[test]
    fn test_sinh() {
        let fun = Sinh::new(0.63123);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"sinh": 0.63123}), serialized);
    }

    #[test]
    fn test_cosh() {
        let fun = Cosh::new(0.63123);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"cosh": 0.63123}), serialized);
    }

    #[test]
    fn test_degrees() {
        let fun = Degrees::new(0.63123);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"degrees": 0.63123}), serialized);
    }

    #[test]
    fn test_add() {
        let fun = Add::new(Array::from(vec![1, 2, 3]));
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"add": [1, 2, 3]}), serialized);
    }

    #[test]
    fn test_bitand() {
        let fun = BitAnd::new(Array::from(vec![1, 1, 0]));
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"bitand": [1, 1, 0]}), serialized);
    }

    #[test]
    fn test_bitor() {
        let fun = BitOr::new(Array::from(vec![1, 1, 0]));
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"bitor": [1, 1, 0]}), serialized);
    }

    #[test]
    fn test_bitxor() {
        let fun = BitXor::new(Array::from(vec![1, 1, 0]));
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"bitxor": [1, 1, 0]}), serialized);
    }

    #[test]
    fn test_divide() {
        let fun = Divide::new(Array::from(vec![2, 4, 1]));
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"divide": [2, 4, 1]}), serialized);
    }

    #[test]
    fn test_bitnot() {
        let fun = BitNot::new(2);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"bitnot": 2}), serialized);
    }

    #[test]
    fn test_ceil() {
        let fun = Ceil::new(4.20);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"ceil": 4.2}), serialized);
    }

    #[test]
    fn test_floor() {
        let fun = Floor::new(4.20);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"floor": 4.2}), serialized);
    }

    #[test]
    fn test_exp() {
        let fun = Exp::new(2);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"exp": 2}), serialized);
    }

    #[test]
    fn test_hypot() {
        let hypot = Hypot::new(3, 2);

        let query = Query::from(hypot);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"hypot": 3, "b": 2}), serialized);
    }

    #[test]
    fn test_hypot_isosceles() {
        let hypot = Hypot::isosceles(3);

        let query = Query::from(hypot);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"hypot": 3}), serialized);
    }

    #[test]
    fn test_ln() {
        let fun = Ln::new(4.20);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"ln": 4.2}), serialized);
    }

    #[test]
    fn test_log() {
        let fun = Log::new(4.20);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"log": 4.2}), serialized);
    }

    #[test]
    fn test_max() {
        let fun = Max::new(Array::from(vec![1, 2]));
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"max": [1, 2]}), serialized);
    }

    #[test]
    fn test_min() {
        let fun = Min::new(Array::from(vec![1, 2]));
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"min": [1, 2]}), serialized);
    }

    #[test]
    fn test_modulo() {
        let fun = Modulo::new(Array::from(vec![1, 2]));
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"modulo": [1, 2]}), serialized);
    }

    #[test]
    fn test_multiply() {
        let fun = Multiply::new(Array::from(vec![1, 2]));
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"multiply": [1, 2]}), serialized);
    }

    #[test]
    fn test_subtract() {
        let fun = Subtract::new(Array::from(vec![1, 2]));
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"subtract": [1, 2]}), serialized);
    }

    #[test]
    fn test_pow() {
        let fun = Pow::new(2, 16);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"pow": 2, "exp": 16}), serialized);
    }

    #[test]
    fn test_radians() {
        let fun = Radians::new(90.90);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"radians": 90.90}), serialized);
    }

    #[test]
    fn test_round() {
        let fun = Round::new(4.20);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"round": 4.2}), serialized);
    }

    #[test]
    fn test_round_with_precision() {
        let mut fun = Round::new(4.20);
        fun.precision(4);

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"round": 4.2, "precision": 4}), serialized);
    }

    #[test]
    fn test_trunc() {
        let fun = Trunc::new(4.20);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"trunc": 4.2}), serialized);
    }

    #[test]
    fn test_trunc_with_precision() {
        let mut fun = Trunc::new(4.20);
        fun.precision(4);

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"trunc": 4.2, "precision": 4}), serialized);
    }

    #[test]
    fn test_sign() {
        let fun = Sign::new(-232);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({"sign": -232}), serialized);
    }

    #[test]
    fn test_sqrt() {
        let fun = Sqrt::new(4);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        assert_eq!(json!({ "sqrt": 4 }), serialized);
    }
}
