use crate::expr::Expr;
use std::{borrow::Cow, collections::BTreeMap};

/// The `Let` function binds one or more variables to a single value or
/// expression.
///
/// When multiple variables are defined, the evaluation is from left
/// to right. Variables which have previously been defined may be used to define
/// future variables. Variables are lexically scoped to the expression passed
/// via the in parameter. The value of a variable can be referenced with
/// Var(varname) syntax.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/basic/let).
#[derive(Debug, Clone, Serialize)]
pub struct Let<'a> {
    #[serde(rename = "let")]
    bindings: BTreeMap<Cow<'a, str>, Expr<'a>>,
    #[serde(rename = "in")]
    in_expr: Expr<'a>,
}

/// A single binding to be used in a `Let` query.
#[derive(Debug, Clone, Serialize)]
pub struct Binding<'a>(Cow<'a, str>, Expr<'a>);

impl<'a> Binding<'a> {
    pub fn new<V, E>(variable: V, expr: E) -> Self
    where
        V: Into<Cow<'a, str>>,
        E: Into<Expr<'a>>,
    {
        Binding(variable.into(), expr.into())
    }
}

impl<'a> Let<'a> {
    /// Set bindings to be available in the given `Expr`.
    pub fn bindings<B, E>(bindings: B, in_expr: E) -> Self
    where
        B: IntoIterator<Item = Binding<'a>>,
        E: Into<Expr<'a>>,
    {
        let bindings = bindings
            .into_iter()
            .map(|binding| (binding.0, binding.1))
            .collect();

        let in_expr = in_expr.into();

        Self { bindings, in_expr }
    }
}
