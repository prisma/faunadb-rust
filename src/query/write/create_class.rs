use crate::{
    expr::{ClassPermission, Expr, Object},
    query::Query,
};
use std::borrow::Cow;

boxed_query!(CreateClass);

/// The `CreateClass` function is used to create a class which groups instance
/// objects.
///
/// Once the class has been created, it is possible to create instances
/// in the class. You cannot create a class and insert instances into that class
/// in the same transaction.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/write/createclass).
#[derive(Debug, Serialize, Clone)]
pub struct CreateClass<'a> {
    create_class: ClassParams<'a>,
}

impl<'a> CreateClass<'a> {
    pub fn new(params: ClassParams<'a>) -> Self {
        Self {
            create_class: params,
        }
    }
}

#[derive(Debug, Default, Serialize, Clone)]
struct ClassParamsInternal<'a> {
    name: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Expr<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    history_days: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl_days: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<ClassPermission<'a>>,
}

#[derive(Debug, Default, Serialize, Clone)]
pub struct ClassParams<'a> {
    object: ClassParamsInternal<'a>,
}

impl<'a> ClassParams<'a> {
    /// The name of a class. Classes cannot be named any of the following
    /// reserved words: `events`, `set`, `self`, `instances`, or `_.`
    pub fn new<S>(name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            object: ClassParamsInternal {
                name: name.into(),
                ..Default::default()
            },
        }
    }

    /// User-defined metadata for the class. It is provided for the
    /// developer to store information at the class level.
    pub fn data(&mut self, data: Object<'a>) -> &mut Self {
        self.object.data = Some(Expr::from(data));
        self
    }

    /// The number of days instance history is retained for this class. Without
    /// setting the value retains this class' history forever. Not setting
    /// history_days retains this class’s history forever.
    pub fn history_days(&mut self, days: u64) -> &mut Self {
        self.object.history_days = Some(days);
        self
    }

    /// The number of days instances are retained for this class. Instances
    /// which have not been updated within the configured TTL duration are
    /// removed. Not setting the `ttl_days` retains instances forever.
    pub fn ttl_days(&mut self, days: u64) -> &mut Self {
        self.object.ttl_days = Some(days);
        self
    }

    /// Provides the ability to enable permissions at the class level.
    pub fn permissions(&mut self, permissions: ClassPermission<'a>) -> &mut Self {
        self.object.permissions = Some(permissions);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{prelude::*, test_utils::*};
    use serde_json::{self, json};

    #[test]
    fn test_create_class_expr() {
        let mut permission = ClassPermission::default();
        permission.read(Level::public());

        let mut params = ClassParams::new("test");
        params.history_days(10);
        params.permissions(permission);

        let query = Query::from(CreateClass::new(params));
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "create_class": {
                "object": {
                    "history_days": 10,
                    "name": "test",
                    "permissions": { "object": { "read": "public" } },
                }
            }
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_create_class_eval() {
        let mut permission = ClassPermission::default();
        permission.read(Level::public());

        let mut data = Object::default();
        data.insert("meow", true);

        let class_name = gen_db_name();

        let mut params = ClassParams::new(&class_name);
        params.history_days(10);
        params.ttl_days(3);
        params.permissions(permission);
        params.data(data);

        with_database(|_| {
            let response = CLIENT.query(CreateClass::new(params)).unwrap();
            let res = response.resource.as_object().unwrap();

            let history_days = res.get("history_days").and_then(|res| res.as_u64());
            let ttl_days = res.get("ttl_days").and_then(|res| res.as_u64());
            let name = res.get("name").and_then(|res| res.as_str());

            let permissions = res
                .get("permissions")
                .and_then(|res| res.as_object())
                .unwrap();

            let data = res.get("data").and_then(|res| res.as_object()).unwrap();

            assert_eq!(history_days, Some(10));
            assert_eq!(ttl_days, Some(3));
            assert_eq!(name, Some(class_name.as_str()));

            assert_eq!(data.get("meow").and_then(|res| res.as_bool()), Some(true));

            assert_eq!(
                permissions.get("read").and_then(|res| res.as_str()),
                Some("public")
            );
        });
    }
}
