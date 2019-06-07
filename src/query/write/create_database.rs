use crate::{error::Error, expr::Object, query::Query, FaunaResult};
use std::borrow::Cow;

query!(CreateDatabase);

/// The `CreateDatabase` function adds a new database to the cluster with the
/// specified parameters.
///
/// It requires an admin key for authentication.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/write/createdatabase)
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CreateDatabase<'a> {
    create_database: DatabaseParams<'a>,
}

#[derive(Debug, Default, Serialize, Clone, Deserialize)]
#[doc(hidden)]
pub struct DatabaseParamsInternal<'a> {
    name: Cow<'a, str>,
    api_version: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Object<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<u16>,
}

#[derive(Debug, Default, Serialize, Clone, Deserialize)]
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
                api_version: 2.0,
                ..Default::default()
            },
        }
    }

    pub fn api_version(&mut self, version: f64) -> &mut Self {
        self.object.api_version = version;
        self
    }

    pub fn data(&mut self, data: Object<'a>) -> &mut Self {
        self.object.data = Some(data);
        self
    }

    pub fn priority(&mut self, priority: u16) -> FaunaResult<&mut Self> {
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
    use super::*;
    use serde_json::{self, json};

    #[test]
    fn test_create_database() {
        let mut params = DatabaseParams::new("test");
        params.priority(10).unwrap();

        let query = Query::from(CreateDatabase::new(params));
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "create_database": {
                "object": {
                    "name": "test",
                    "api_version": 2.0,
                    "priority": 10,
                }
            }
        });

        assert_eq!(expected, serialized);
    }
}
