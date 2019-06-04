use crate::expr::{ClassPermission, Expr, Object};

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
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Expr<'a>>,
    history_days: Option<u64>,
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
        S: Into<&'a str>,
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
