use crate::{error::Error, expr::Object, FaunaResult};

#[derive(Debug, Serialize, Clone)]
pub struct CreateDatabase<'a> {
    create_database: DatabaseParams<'a>,
}

#[derive(Debug, Default, Serialize, Clone)]
pub struct DatabaseParamsInternal<'a> {
    name: &'a str,
    api_version: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Object<'a>>,
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
        S: Into<&'a str>,
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
