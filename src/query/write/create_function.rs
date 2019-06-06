use crate::{
    expr::{Expr, Object},
    query::Query,
};
use std::borrow::Cow;

boxed_query!(CreateFunction);

/// The `CreateFunction` operation adds a new user-defined function with the
/// specified parameters.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/write/createfunction).
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CreateFunction<'a> {
    create_function: FunctionParams<'a>,
}

impl<'a> CreateFunction<'a> {
    pub fn new(params: FunctionParams<'a>) -> Self {
        Self {
            create_function: params,
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
struct FunctionParamsInternal<'a> {
    name: Cow<'a, str>,
    body: Expr<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Expr<'a>>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct FunctionParams<'a> {
    object: FunctionParamsInternal<'a>,
}

impl<'a> FunctionParams<'a> {
    pub fn new<S, E>(name: S, body: E) -> Self
    where
        S: Into<Cow<'a, str>>,
        E: Into<Expr<'a>>,
    {
        Self {
            object: FunctionParamsInternal {
                name: name.into(),
                body: body.into().as_quoted(),
                data: None,
            },
        }
    }

    /// User-defined metadata for the function. It is provided for the
    /// developer to store information at the function level.
    pub fn data(&mut self, data: Object<'a>) -> &mut Self {
        self.object.data = Some(Expr::from(data));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde_json::{self, json};

    #[test]
    fn test_create_function() {
        let params = FunctionParams::new(
            "double",
            Lambda::new(
                "x",
                Add::new(Array::from(vec![Var::new("x"), Var::new("x")])),
            ),
        );

        let query = Query::from(CreateFunction::new(params));
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "create_function": {
                "object": {
                    "body": {
                        "@query": {
                            "expr": {
                                "add": [{"var": "x"}, {"var": "x"}]
                            },
                            "lambda": "x"
                        }
                    },
                    "name": "double"
                }
            }
        });

        assert_eq!(expected, serialized);
    }
}
