use crate::expr::Expr;

/// The `If` function evaluates and returns `if_true` or `if_false` depending on
/// the value of the `cond` expression. If the `cond` expression evaluates to
/// anything other than a `Boolean`, `If` returns an `invalid argument` error.
#[derive(Debug, Clone)]
pub struct If<'a> {
    pub(crate) cond: Expr<'a>,
    pub(crate) if_true: Expr<'a>,
    pub(crate) if_false: Expr<'a>,
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
