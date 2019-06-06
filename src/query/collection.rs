use crate::{
    expr::Expr,
    query::{basic::Lambda, Query},
};

query!(Append, Map);

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
}
