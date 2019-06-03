use crate::query::Query;

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
#[derive(Serialize, Debug)]
pub struct Do<'a>(Vec<Query<'a>>);

impl<'a> Do<'a> {
    /// Create a new `Do` query.
    pub fn new(first_query: impl Into<Query<'a>>) -> Self {
        Do(vec![first_query.into()])
    }

    /// Add a query to the end of the execution pipeline.
    pub fn push(&mut self, q: impl Into<Query<'a>>) -> &mut Self {
        self.0.push(q.into());
        self
    }
}
