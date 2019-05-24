use crate::expr::Object;

#[derive(Debug, Serialize)]
pub enum Params<'a> {
    #[serde(rename = "object")]
    Object(Object<'a>),
}

impl<'a> From<Object<'a>> for Params<'a> {
    fn from(obj: Object<'a>) -> Self {
        Params::Object(obj)
    }
}

#[derive(Debug, Serialize)]
pub struct Query<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<Create<'a>>,
    params: Params<'a>,
}

impl<'a> Query<'a> {
    pub fn create<O>(query: Create<'a>, params: O) -> Self
    where
        O: Into<Params<'a>>,
    {
        Self {
            create: Some(query),
            params: params.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum QueryType<'a> {
    #[serde(rename = "create")]
    Create(Create<'a>),
}

impl<'a> From<Create<'a>> for QueryType<'a> {
    fn from(create: Create<'a>) -> Self {
        QueryType::Create(create)
    }
}

#[derive(Debug, Serialize)]
pub struct Create<'a> {
    #[serde(rename = "@ref")]
    ref_: Ref<'a>,
}

impl<'a> Create<'a> {
    pub fn instance(class: Class<'a>) -> Self {
        let mut ref_ = Ref::new(class.id);
        ref_.class(class);

        Self { ref_ }
    }
}

#[derive(Debug, Serialize)]
pub struct Ref<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    class: Option<Box<Class<'a>>>,
    id: &'a str,
}

impl<'a> Ref<'a> {
    fn new(id: &'a str) -> Self {
        Self { id, class: None }
    }

    fn class(&mut self, class: Class<'a>) -> &mut Self {
        self.class = Some(Box::new(class));
        self
    }
}

#[derive(Debug, Serialize)]
pub struct Class<'a> {
    #[serde(rename = "@ref")]
    ref_: Ref<'a>,
    #[serde(skip_serializing)]
    id: &'a str,
}

impl<'a> Class<'a> {
    pub fn new(id: &'a str) -> Self {
        Self {
            ref_: Ref::new("classes"),
            id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::{self, json};

    #[test]
    fn test_create_instance() {
        let mut params = Object::new();
        params.insert("test_field", "test_value");

        let mut data = Object::new();
        data.insert("data", params);

        let query = Query::create(Create::instance(Class::new("test")), data);

        let result = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "params": {
                "object": {
                    "data": {
                        "object": {
                            "test_field": "test_value"
                        }
                    }
                }
            },
            "create": {
                "@ref": {
                    "class": {
                        "@ref": {
                            "id": "classes"
                        }
                    },
                    "id": "test"
                }
            }
        });

        assert_eq!(expected, result);
    }
}
