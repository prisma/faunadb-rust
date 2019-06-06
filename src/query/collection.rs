use crate::{
    expr::Expr,
    query::{basic::Lambda, Query},
};

query!(Map);

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
}
