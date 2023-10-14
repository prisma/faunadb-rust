use crate::{
    error::Error,
    expr::{Expr, Object},
    query::Query,
};
use std::borrow::Cow;

query!(CreateDatabase);

/// The `CreateDatabase` function adds a new database to the cluster with the
/// specified parameters.
///
/// It requires an admin key for authentication.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/write/createdatabase)
#[derive(Debug, Serialize, Clone)]
pub struct CreateDatabase<'a> {
    create_database: DatabaseParams<'a>,
}

#[derive(Debug, Default, Serialize, Clone)]
#[doc(hidden)]
pub struct DatabaseParamsInternal<'a> {
    name: Cow<'a, str>,
    api_version: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Expr<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<u16>,
}

#[derive(Debug, Default, Serialize, Clone)]
pub struct DatabaseParams<'a> {
    object: DatabaseParamsInternal<'a>,
}

impl<'a> CreateDatabase<'a> {
    pub fn new(params: DatabaseParams<'a>) -> Self {
        Self {
            create_database: params,
        }
    }
}

impl<'a> DatabaseParams<'a> {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            object: DatabaseParamsInternal {
                name: name.into(),
                api_version: Cow::from("2.0"),
                ..Default::default()
            },
        }
    }

    pub fn api_version(&mut self, version: impl Into<Cow<'a, str>>) -> &mut Self {
        self.object.api_version = version.into();
        self
    }

    pub fn data(&mut self, data: Object<'a>) -> &mut Self {
        self.object.data = Some(Expr::from(data));
        self
    }

    pub fn priority(&mut self, priority: u16) -> crate::Result<&mut Self> {
        if priority == 0 || priority > 500 {
            return Err(Error::RequestDataFailure(
                "Priority should be a number between 1 and 500",
            ));
        }

        self.object.priority = Some(priority);

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{prelude::*, test_utils::CLIENT};
    use serde_json::{self, json};
    use std::panic;

    #[test]
    fn test_create_database_expr() {
        let mut params = DatabaseParams::new("test");
        params.priority(10).unwrap();

        let query = Query::from(CreateDatabase::new(params));
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "create_database": {
                "object": {
                    "name": "test",
                    "api_version": "2.0",
                    "priority": 10,
                }
            }
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_create_database_eval() {
        let mut data = Object::default();
        data.insert("foo", "bar");

        let db_name = "test";
        let mut params = DatabaseParams::new(db_name);
        params.priority(10).unwrap();
        params.data(data);

        let result = panic::catch_unwind(|| {
            let response = CLIENT.query(CreateDatabase::new(params)).unwrap();
            let res = response.resource;

            assert_eq!(res["name"].as_str(), Some(db_name));
            assert_eq!(res["priority"].as_u64(), Some(10));

            assert_eq!(
                res["ref"].as_reference().unwrap().path(),
                Ref::database(db_name).path()
            );

            assert_eq!(res["data"]["foo"].as_str(), Some("bar"));

            assert!(res["ts"].is_number());
        });

        CLIENT.query(Delete::new(Ref::database(db_name))).unwrap();

        result.unwrap();
    }
}
