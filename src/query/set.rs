use crate::{
    expr::{Array, Expr},
    query::Query,
};

query![Difference, Distinct, Intersection, Join, Match, Union];

/// The `Difference` function returns a `SetRef` object that represents all elements
/// in the first `SetRef` which are not in the difference `SetRef`(s).
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/set/difference)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Difference<'a> {
    difference: Array<'a>,
}

impl<'a> Difference<'a> {
    /// Get the difference that represents all elements in the `left` `SetRef`
    /// that are not in the `right` `SetRef`.
    pub fn new(left: impl Into<Expr<'a>>, right: impl Into<Expr<'a>>) -> Self {
        Self {
            difference: Array::from(vec![left.into(), right.into()]),
        }
    }

    /// Add a `SetRef` expression to be used in the `Difference`.
    pub fn push(&mut self, e: impl Into<Expr<'a>>) -> &mut Self {
        self.difference.push(e.into());
        self
    }
}

impl<'a, A> From<A> for Difference<'a>
where
    A: Into<Array<'a>>,
{
    fn from(a: A) -> Self {
        Self {
            difference: a.into(),
        }
    }
}

/// The `Distinct` function returns a SetRef object that represents all of the
/// unique elements in the provided SetRef.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/set/distinct)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Distinct<'a> {
    distinct: Expr<'a>,
}

impl<'a> Distinct<'a> {
    pub fn new(e: impl Into<Expr<'a>>) -> Self {
        Self { distinct: e.into() }
    }
}

/// The `Intersection` function returns a `SetRef` object that contains the elements
/// that appears in every input `SetRef`.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/set/intersection)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Intersection<'a> {
    intersection: Array<'a>,
}

impl<'a> Intersection<'a> {
    /// Get the intersection that represents all elements in the `left` `SetRef`
    /// that are in the `right` `SetRef`.
    pub fn new(left: impl Into<Expr<'a>>, right: impl Into<Expr<'a>>) -> Self {
        Self {
            intersection: Array::from(vec![left.into(), right.into()]),
        }
    }

    /// Add a `SetRef` expression to be used in the `Intersection`.
    pub fn push(&mut self, e: impl Into<Expr<'a>>) -> &mut Self {
        self.intersection.push(e.into());
        self
    }
}

impl<'a, A> From<A> for Intersection<'a>
where
    A: Into<Array<'a>>,
{
    fn from(a: A) -> Self {
        Self {
            intersection: a.into(),
        }
    }
}

/// The `Join` function finds all index tuples from the `source` SetRef and uses the
/// source's values to be retrieved from the `detail` index terms.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/set/intersection)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Join<'a> {
    join: Expr<'a>,
    with: Expr<'a>,
}

impl<'a> Join<'a> {
    pub fn new(source: impl Into<Expr<'a>>, detail: impl Into<Expr<'a>>) -> Self {
        Self {
            join: source.into(),
            with: detail.into(),
        }
    }
}

/// The `Match` function finds the "search terms" provided to `Match` in the
/// requested index.
///
/// The `search_terms` must be identical to the terms in the index, including both
/// the value of all terms and number of terms. If the index is configured with
/// no terms, then the search_terms argument should be omitted. If the index is
/// configured with multiple terms, then the "search terms" should be an array
/// of values.
///
/// When calling Match through paginate, the results are returned as a
/// Collection of type Page. If no matching element is found an empty collection
/// is returned.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/set/match)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Match<'a> {
    #[serde(rename = "match")]
    match_: Expr<'a>,
    terms: Expr<'a>,
}

impl<'a> Match<'a> {
    pub fn new(match_: impl Into<Expr<'a>>, terms: impl Into<Expr<'a>>) -> Self {
        Self {
            match_: match_.into(),
            terms: terms.into(),
        }
    }
}

/// The `Union` function combines the results one or more `SetRef` objects.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/set/union)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Union<'a> {
    union: Array<'a>,
}

impl<'a> Union<'a> {
    /// Get the union that represents all elements in the `left` `SetRef`
    /// and all elements in the `right` `SetRef`.
    pub fn new(left: impl Into<Expr<'a>>, right: impl Into<Expr<'a>>) -> Self {
        Self {
            union: Array::from(vec![left.into(), right.into()]),
        }
    }

    /// Add a `SetRef` expression to be used in the `Union`.
    pub fn push(&mut self, e: impl Into<Expr<'a>>) -> &mut Self {
        self.union.push(e.into());
        self
    }
}

impl<'a, A> From<A> for Union<'a>
where
    A: Into<Array<'a>>,
{
    fn from(a: A) -> Self {
        Self { union: a.into() }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde_json::{self, json};

    #[test]
    fn test_difference() {
        let fun = Difference::new(
            Match::new(Index::find("spells_by_element"), "fire"),
            Match::new(Index::find("spells_by_element"), "water"),
        );

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "difference": [
                {"match": {"index": "spells_by_element"}, "terms": "fire"},
                {"match": {"index": "spells_by_element"}, "terms": "water"},
            ]
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_distinct() {
        let fun = Distinct::new(Match::new(Index::find("spells_by_element"), Expr::null()));

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "distinct": {"match": {"index": "spells_by_element"}, "terms": null},
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_intersection() {
        let fun = Intersection::new(
            Match::new(Index::find("spells_by_element"), "fire"),
            Match::new(Index::find("spells_by_element"), "water"),
        );

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "intersection": [
                {"match": {"index": "spells_by_element"}, "terms": "fire"},
                {"match": {"index": "spells_by_element"}, "terms": "water"},
            ]
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_join() {
        let mut owner = Ref::instance("wizard");
        owner.set_class("characters");

        let fun = Join::new(
            Match::new(Index::find("spellbooks_by_owner"), owner),
            Index::find("spells_by_spellbook"),
        );

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "join": {
                "match": {"index": "spellbooks_by_owner"},
                "terms": {
                    "@ref": {
                        "class": {
                            "@ref": {
                                "class": {
                                    "@ref": {
                                        "id": "classes"
                                    }
                                },
                                "id": "characters"
                            }
                        },
                        "id": "wizard"
                    }
                },
            },
            "with": {"index": "spells_by_spellbook"},
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_match() {
        let fun = Match::new(Index::find("spells_by_element"), "fire");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "match": {"index": "spells_by_element"}, "terms": "fire"
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_union() {
        let fun = Union::new(
            Match::new(Index::find("spells_by_element"), "fire"),
            Match::new(Index::find("spells_by_element"), "water"),
        );

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "union": [
                {"match": {"index": "spells_by_element"}, "terms": "fire"},
                {"match": {"index": "spells_by_element"}, "terms": "water"},
            ]
        });

        assert_eq!(expected, serialized);
    }
}
