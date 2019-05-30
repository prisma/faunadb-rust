use crate::expr::Object;

#[derive(Debug, Serialize)]
pub struct CreateClass<'a> {
    param_object: Object<'a>
}

impl<'a> CreateClass<'a> {
    pub fn new(params: ClassParams<'a>) -> Self {
        Self {
            param_object: Object::from(params),
        }
    }
}

#[derive(Debug, Default)]
pub struct ClassParams<'a> {
    name: &'a str,
    data: Option<Object<'a>>,
    history_days: Option<u64>,
    ttl_days: Option<u64>,
}

impl<'a> ClassParams<'a> {
    pub fn new<S>(name: S) -> Self where S: Into<&'a str> {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn data(&mut self, data: Object<'a>) -> &mut Self {
        self.data = Some(data);
        self
    }

    pub fn history_days(&mut self, days: u64) -> &mut Self {
        self.history_days = Some(days);
        self
    }

    pub fn ttl_days(&mut self, days: u64) -> &mut Self {
        self.ttl_days = Some(days);
        self
    }
}

impl<'a> From<ClassParams<'a>> for Object<'a> {
    fn from(cp: ClassParams<'a>) -> Self {
        let mut obj = Object::new();
        obj.insert("name", cp.name);

        if let Some(data) = cp.data {
            obj.insert("data", data);
        }

        if let Some(days) = cp.history_days {
            obj.insert("history_days", days);
        }

        if let Some(days) = cp.ttl_days {
            obj.insert("ttl_days", days);
        }

        obj
    }
}
