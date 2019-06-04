use crate::expr::Expr;
use std::borrow::Cow;

/// The `Lambda` function is an anonymous function that performs lazy execution
/// of custom code. It allows you to organize and execute almost any of the
/// Fauna Query Language statements.
///
/// A `Lambda` can take zero or more arguments. `Lambda`s that
/// define multiple parameters use a `params` array to define the arguments. In
/// this case, the items inside the `params` array are the arguments, not the
/// array itself. The `params` array must have the same number of elements as
/// the Lambda function expects, or an `_` (i.e., underscore) argument to drop
/// the extra arguments in the array. Otherwise, it will return an error.
///
/// The `Lambda` arguments may be accessed inside the `Lambda` code using the
/// [Var](struct.Var.html) statement.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/basic/lambda).
#[derive(Serialize, Debug, Clone)]
pub struct Lambda<'a> {
    #[serde(rename = "lambda")]
    params: Cow<'a, [&'a str]>, // moo
    expr: Expr<'a>,
}

impl<'a> Lambda<'a> {
    pub fn new<P, E>(params: P, expr: E) -> Self
    where
        P: Into<Cow<'a, [&'a str]>>,
        E: Into<Expr<'a>>,
    {
        Self {
            params: params.into(),
            expr: expr.into(),
        }
    }
}
