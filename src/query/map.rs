use crate::{expr::Expr, query::Lambda};

/// The `Map` function applies a [Lambda](struct.Lambda.html) serially to each
/// member of the collection and returns the results of each application in a
/// new collection of the same type. Later invocations of the `Lambda` function
/// can see the results of earlier invocations.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/collection/map).
#[derive(Serialize, Clone, Debug)]
pub struct Map<'a> {
    collection: Expr<'a>,
    map: Lambda<'a>,
}

impl<'a> Map<'a> {
    pub fn new<E>(collection: E, lambda: Lambda<'a>) -> Self
    where
        E: Into<Expr<'a>>,
    {
        Self {
            collection: collection.into(),
            map: lambda,
        }
    }
}
