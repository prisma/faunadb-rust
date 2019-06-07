//! Basic functions
use crate::{
    expr::{Expr, Ref},
    query::Query,
};
use chrono::{DateTime, Utc};
use std::{borrow::Cow, collections::BTreeMap};

// Implements From<fun> for Query
query![At, Call, If, Do, Let, Var, Lambda];

/// The At function executes a temporal query, a query which examines the data
/// in the past.
///
/// The `timestamp` parameter determines the data available for viewing by
/// creating a virtual snapshot of the data which was current at that date and
/// time. All reads from the associated `expression` is then executed on that
/// virtual snapshot. In contrast, all write operations must be executed at the
/// current time. Attempting a write operation at any other time produces an
/// error.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/basic/at);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct At<'a> {
    #[serde(rename = "at")]
    timestamp: Expr<'a>,
    #[serde(rename = "expr")]
    expression: Expr<'a>,
}

impl<'a> At<'a> {
    pub fn new(timestamp: DateTime<Utc>, expression: impl Into<Expr<'a>>) -> Self {
        Self {
            timestamp: Expr::from(timestamp),
            expression: expression.into(),
        }
    }
}

/// The `Call` function executes a user-defined function previously defined with
/// the CreateFunction function.
///
/// The Call function takes a variable length list of arguments which must match
/// the type and number of the function being called. These arguments are
/// provided to the function being executed by `Call`.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/basic/call);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Call<'a> {
    call: Expr<'a>,
    arguments: Expr<'a>,
}

impl<'a> Call<'a> {
    /// The `arguments` must evaluate to the type and number of arguments of the
    /// function being called.
    pub fn new(function: Ref<'a>, arguments: impl Into<Expr<'a>>) -> Self {
        Self {
            call: Expr::from(function),
            arguments: arguments.into(),
        }
    }
}

/// The `If` function evaluates and returns `if_true` or `if_false` depending on
/// the value of the `cond` expression.
///
/// If the `cond` expression evaluates to
/// anything other than a `Boolean`, `If` returns an `invalid argument` error.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/basic/if);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct If<'a> {
    #[serde(rename = "if")]
    cond: Expr<'a>,
    #[serde(rename = "then")]
    if_true: Expr<'a>,
    #[serde(rename = "else")]
    if_false: Expr<'a>,
}

impl<'a> If<'a> {
    /// Create a new `If` conditional. The `cond` parameter should always return
    /// a `Boolean` expression.
    pub fn cond(
        cond: impl Into<Expr<'a>>,
        if_true: impl Into<Expr<'a>>,
        if_false: impl Into<Expr<'a>>,
    ) -> Self {
        Self {
            cond: cond.into(),
            if_true: if_true.into(),
            if_false: if_false.into(),
        }
    }
}

/// The `Do` function evaluates a list of expressions which are provided as
/// arguments.
///
/// This evaluation occurs sequentially, from left to right, ensuring
/// that modifications made by earlier expressions are seen by later
/// expressions.
///
/// If one of the expressions evaluated by 'Do' returns an error, the
/// current transaction is terminated and none of the expressions' effects are
/// persisted in the database.
///
/// If all of the expressions executed by 'Do' succeed,
/// only the results of the last statements executed are returned.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/basic/do).
#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Do<'a> {
    #[serde(rename = "do")]
    queries: Vec<Expr<'a>>,
}

impl<'a> Do<'a> {
    /// Create a new `Do` query.
    pub fn new(first_expr: impl Into<Expr<'a>>) -> Self {
        Do {
            queries: vec![first_expr.into()],
        }
    }

    /// Add a query to the end of the execution pipeline.
    pub fn push(&mut self, q: impl Into<Expr<'a>>) -> &mut Self {
        self.queries.push(q.into());
        self
    }
}

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
#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Lambda<'a> {
    #[serde(rename = "lambda")]
    params: Expr<'a>,
    expr: Expr<'a>,
}

impl<'a> Lambda<'a> {
    pub fn new(params: impl Into<Expr<'a>>, expr: impl Into<Expr<'a>>) -> Self {
        Self {
            params: params.into(),
            expr: expr.into(),
        }
    }
}

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Let<'a> {
    #[serde(rename = "let")]
    bindings: BTreeMap<Cow<'a, str>, Expr<'a>>,
    #[serde(rename = "in")]
    in_expr: Expr<'a>,
}

/// A single binding to be used in a `Let` query.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Evaluate and return the value stored in a named variable.
///
/// The `Var` statement can only be used inside other statements, such
/// as [Let](struct.Let.html) or [Lambda](struct.Lambda.html).
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/basic/var)
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Var<'a> {
    var: Cow<'a, str>,
}

impl<'a> Var<'a> {
    pub fn new<V>(var: V) -> Self
    where
        V: Into<Cow<'a, str>>,
    {
        Self { var: var.into() }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        prelude::*,
        query::{misc::Classes, read::Get, write::Delete},
    };
    use chrono::{offset::TimeZone, Utc};
    use serde_json::{self, json};

    #[test]
    fn test_at() {
        let fun = At::new(Utc.timestamp(60, 0), Classes::all());
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "at": {"@ts": "1970-01-01T00:01:00Z"},
            "expr": {"classes": null}
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_do() {
        let mut do_many = Do::new(Get::instance(Ref::instance("musti")));
        do_many.push(Delete::new(Ref::instance("musti")));

        let query = Query::from(do_many);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "do": [
                {"get": {"@ref": {"id": "musti"}}},
                {"delete": {"@ref": {"id": "musti"}}},
            ]
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_if() {
        let query = Query::from(If::cond(true, "is true", "is false"));
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "if": true,
            "then": "is true",
            "else": "is false",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_let_var() {
        let let_var = Let::bindings(
            vec![Binding::new("cat", If::cond(true, "Musti", "Naukio"))],
            Var::new("cat"),
        );

        let query = Query::from(let_var);

        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "let": {"cat": {"if": true, "then": "Musti", "else": "Naukio"}},
            "in": {"var": "cat"},
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_lambda() {
        let lambda = Lambda::new("cat", Var::new("cat"));
        let query = Query::from(lambda);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "lambda": "cat",
            "expr": {"var": "cat"},
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_call() {
        let fun = Call::new(Ref::function("double"), 5);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "call": {
                "@ref": {
                    "class": {
                        "@ref": {
                            "id": "functions"
                        }
                    },
                    "id": "double"
                }
            },
            "arguments": 5
        });

        assert_eq!(expected, serialized);
    }
}
