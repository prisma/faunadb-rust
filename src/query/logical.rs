use crate::expr::{Expr, Ref};

/// The `And` function computes the conjunction of a list of boolean values,
/// returning `true` if all elements are "true", and `false` otherwise.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/and)
#[derive(Serialize, Debug, Clone, Default)]
pub struct And<'a> {
    and: Vec<Expr<'a>>,
}

impl<'a> And<'a> {
    pub fn new<I, E>(exprs: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<Expr<'a>>,
    {
        And {
            and: exprs.into_iter().map(Into::into).collect(),
        }
    }
}

/// The `Or` function operates on one or more values and returns true if at least
/// one of the values is `true`.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/or)
#[derive(Serialize, Debug, Clone, Default)]
pub struct Or<'a> {
    or: Vec<Expr<'a>>,
}

impl<'a> Or<'a> {
    pub fn new<I, E>(exprs: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<Expr<'a>>,
    {
        Or {
            or: exprs.into_iter().map(Into::into).collect(),
        }
    }
}

/// The `Not` function computes the negation of a boolean value, returning true if
/// its argument is `false`, or `false` if its argument is `true`.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/not)
#[derive(Serialize, Debug, Clone)]
pub struct Not<'a> {
    not: Expr<'a>,
}

impl<'a> Not<'a> {
    pub fn new(expr: impl Into<Expr<'a>>) -> Self {
        Self { not: expr.into() }
    }
}

/// The `Equals` function tests equivalence between a list of values.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/equals)
#[derive(Serialize, Debug, Clone)]
pub struct Equals<'a> {
    equals: Vec<Expr<'a>>,
}

impl<'a> Equals<'a> {
    pub fn new<I, E>(exprs: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<Expr<'a>>,
    {
        Equals {
            equals: exprs.into_iter().map(Into::into).collect(),
        }
    }
}

/// The `Lt` function returns `true` if each specified value is less than the ones
/// following it, and `false` otherwise.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/lt)
#[derive(Serialize, Debug, Clone, Default)]
pub struct Lt<'a> {
    lt: Vec<Expr<'a>>,
}

impl<'a> Lt<'a> {
    pub fn new<I, E>(exprs: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<Expr<'a>>,
    {
        Lt {
            lt: exprs.into_iter().map(Into::into).collect(),
        }
    }
}

/// The `Lte` function returns `true` if each specified value is less than or
/// equal to the ones following it, and `false` otherwise.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/lte)
#[derive(Serialize, Debug, Clone, Default)]
pub struct Lte<'a> {
    lte: Vec<Expr<'a>>,
}

impl<'a> Lte<'a> {
    pub fn new<I, E>(exprs: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<Expr<'a>>,
    {
        Lte {
            lte: exprs.into_iter().map(Into::into).collect(),
        }
    }
}

/// The `Gt` function returns `true` if each specified value is greater than the
/// ones following it, and `false` otherwise.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/gt)
#[derive(Serialize, Debug, Clone, Default)]
pub struct Gt<'a> {
    gt: Vec<Expr<'a>>,
}

impl<'a> Gt<'a> {
    pub fn new<I, E>(exprs: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<Expr<'a>>,
    {
        Gt {
            gt: exprs.into_iter().map(Into::into).collect(),
        }
    }
}

/// The `Gte` function returns `true` if each specified value is greater than or
/// equal to the ones following it, and `false` otherwise.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/gte)
#[derive(Serialize, Debug, Clone, Default)]
pub struct Gte<'a> {
    gte: Vec<Expr<'a>>,
}

impl<'a> Gte<'a> {
    pub fn new<I, E>(exprs: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<Expr<'a>>,
    {
        Gte {
            gte: exprs.into_iter().map(Into::into).collect(),
        }
    }
}

/// The `Contains` function returns `true` if the argument passed as in contains a
/// value at the specified path, and `false` otherwise.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/contains)
#[derive(Serialize, Debug, Clone)]
pub struct Contains<'a> {
    contains: Vec<Expr<'a>>,
    #[serde(rename = "in")]
    in_: Expr<'a>,
}

impl<'a> Contains<'a> {
    pub fn new<I, E, F>(path: I, in_: F) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<Expr<'a>>,
        F: Into<Expr<'a>>,
    {
        Self {
            contains: path.into_iter().map(|e| e.into()).collect(),
            in_: in_.into(),
        }
    }
}

/// The `Exists` function returns boolean `true` if the provided ref exists at the
/// specified timestamp (in the case of an instance), or is non-empty (in the
/// case of a set), and `false` otherwise.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/logical/exists)
#[derive(Serialize, Debug, Clone)]
pub struct Exists<'a> {
    exists: Expr<'a>,
    #[serde(rename = "ts", skip_serializing_if = "Option::is_none")]
    timestamp: Option<Expr<'a>>,
}

impl<'a> Exists<'a> {
    pub fn new(reference: Ref<'a>) -> Self {
        Self {
            exists: Expr::from(reference),
            timestamp: None,
        }
    }
}
