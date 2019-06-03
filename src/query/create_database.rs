use crate::{
    error::Error,
    expr::{Expr, Object},
    FaunaResult,
};

#[derive(Debug, Serialize)]
pub struct CreateDatabase<'a> {
    #[serde(flatten)]
    param_object: Expr<'a>,
}

impl<'a> CreateDatabase<'a> {
    pub fn new(params: DatabaseParams<'a>) -> Self {
        Self {
            param_object: Expr::from(params),
        }
    }
}

#[derive(Debug, Default)]
pub struct DatabaseParams<'a> {
    name: &'a str,
    api_version: f64,
    data: Option<Object<'a>>,
    priority: Option<u16>,
}

impl<'a> DatabaseParams<'a> {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<&'a str>,
    {
        Self {
            name: name.into(),
            api_version: 2.0,
            ..Default::default()
        }
    }

    pub fn api_version(&mut self, version: f64) -> &mut Self {
        self.api_version = version;
        self
    }

    pub fn data(&mut self, data: Object<'a>) -> &mut Self {
        self.data = Some(data);
        self
    }

    pub fn priority(&mut self, priority: u16) -> FaunaResult<&mut Self> {
        if priority == 0 || priority > 500 {
            return Err(Error::RequestDataFailure(
                "Priority should be a number between 1 and 500",
            ));
        }

        self.priority = Some(priority);
        Ok(self)
    }
}

impl<'a> From<DatabaseParams<'a>> for Object<'a> {
    fn from(dp: DatabaseParams<'a>) -> Self {
        let mut obj = Object::default();
        obj.insert("name", dp.name);
        obj.insert("api_version", dp.api_version);

        if let Some(data) = dp.data {
            obj.insert("data", data);
        }

        if let Some(priority) = dp.priority {
            obj.insert("priority", priority);
        }

        obj
    }
}
