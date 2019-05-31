use crate::expr::{Expr, Object, ClassPermission};

#[derive(Debug, Serialize)]
pub struct CreateClass<'a> {
    object: ClassParams<'a>
}

impl<'a> CreateClass<'a> {
    pub fn new(params: ClassParams<'a>) -> Self {
        Self {
            object: params,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct ClassParams<'a> {
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Expr<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    history_days: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl_days: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<ClassPermission<'a>>,
}

impl<'a> ClassParams<'a> {
    pub fn new<S>(name: S) -> Self where S: Into<&'a str> {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn data(&mut self, data: Object<'a>) -> &mut Self {
        self.data = Some(Expr::from(data));
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

    pub fn permissions(&mut self, permissions: ClassPermission<'a>) -> &mut Self {
        self.permissions = Some(permissions);
        self
    }
}
