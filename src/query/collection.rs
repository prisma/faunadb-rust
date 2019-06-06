use crate::{
    expr::Expr,
    query::{basic::Lambda, Query},
};

query!(Append, Map, Drop, Filter);

/// The `Append` function creates a new array that is the result of combining the
/// base Array followed by the `elems`.
///
/// This is a specialized function, and only works with collections of type
/// Array.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/collection/append).
#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct Append<'a> {
    append: Expr<'a>,
    collection: Expr<'a>,
}

impl<'a> Append<'a> {
    pub fn new(base: impl Into<Expr<'a>>, elems: impl Into<Expr<'a>>) -> Self {
        Self {
            append: base.into(),
            collection: elems.into(),
        }
    }
}

/// The `Drop` function returns a new collection of the same type that contains
/// the remaining elements, after `num` have been removed from the head of the
/// collection.
///
/// If `num` is zero or negative, all elements of the collection are
/// returned unmodified.
///
/// When applied to a collection of type page, the returned page’s `before` cursor
/// is adjusted to exclude the dropped elements. As special cases:
///
///  * If `num` is negative, `before` does not change.
///  * Otherwise if all elements from the original page were dropped (including
///    the case where the page was already empty), `before` is set to same value as
///    the original page’s `after`.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/collection/drop).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drop<'a> {
    drop: Expr<'a>,
    collection: Expr<'a>,
}

impl<'a> Drop<'a> {
    /// The `drop` parameter must evaluate to an integer and `collection` to a
    /// collection.
    pub fn new(drop: impl Into<Expr<'a>>, collection: impl Into<Expr<'a>>) -> Self {
        Self {
            drop: drop.into(),
            collection: collection.into(),
        }
    }
}

/// The Filter function applies the Lambda to each member of the collection and
/// returns a new collection of the same type containing only those elements for
/// which the lambda returns true. Providing a lambda function which does not
/// return a Boolean results in an "invalid argument" error. If a Page is
/// passed, its decorated fields are preserved in the result.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/collection/filter).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter<'a> {
    filter: Expr<'a>,
    collection: Expr<'a>,
}

impl<'a> Filter<'a> {
    pub fn new(filter: Lambda<'a>, collection: impl Into<Expr<'a>>) -> Self {
        Self {
            filter: Expr::from(filter),
            collection: collection.into(),
        }
    }
}

/// The `Map` function applies a [Lambda](struct.Lambda.html) serially to each
/// member of the collection and returns the results of each application in a
/// new collection of the same type. Later invocations of the `Lambda` function
/// can see the results of earlier invocations.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/collection/map).
#[derive(Serialize, Clone, Debug, Deserialize)]
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

#[cfg(test)]
mod tests {
    use crate::{prelude::*, query::basic::Var};
    use serde_json::{self, json};

    #[test]
    fn test_map() {
        let map = Map::new(
            Array::from(vec!["Musti", "Naukio"]),
            Lambda::new("cat", Var::new("cat")),
        );

        let query = Query::from(map);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "collection": ["Musti", "Naukio"],
            "map": {
                "lambda": "cat",
                "expr": {"var": "cat"},
            }
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_append() {
        let fun = Append::new(
            Array::from(vec!["Musti", "Naukio"]),
            Array::from(vec!["Musmus", "Naunau"]),
        );

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "append": ["Musti", "Naukio"],
            "collection": ["Musmus", "Naunau"],
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_drop() {
        let fun = Drop::new(2, Array::from(vec![1, 2, 3]));
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "drop": 2,
            "collection": [1, 2, 3],
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_filter() {
        let fun = Filter::new(
            Lambda::new("x", Gt::new(Var::new("x"), 2)),
            Array::from(vec![1, 2, 3]),
        );

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "filter": {
                "lambda": "x",
                "expr": {"gt": [{ "var": "x" }, 2]}
            },
            "collection": [1, 2, 3],
        });

        assert_eq!(expected, serialized);
    }
}
