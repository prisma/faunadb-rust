use std::borrow::Cow;

/// Evaluate and return the value stored in a named variable.
///
/// The `Var` statement can only be used inside other statements, such
/// as [Let](struct.Let.html) or [Lambda](struct.Lambda.html).
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/basic/var)
#[derive(Debug, Serialize, Clone)]
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
