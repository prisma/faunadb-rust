use crate::{
    expr::{Expr, Ref},
    query::Query,
};

query![Classes, Databases];

#[derive(Serialize, Debug, Clone, Default)]
pub struct Classes<'a> {
    classes: Option<Expr<'a>>,
}

impl<'a> Classes<'a> {
    pub fn all() -> Self {
        Self::default()
    }

    pub fn from_database(database: Ref<'a>) -> Self {
        Self {
            classes: Some(Expr::from(database)),
        }
    }
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Databases<'a> {
    databases: Option<Expr<'a>>,
}

impl<'a> Databases<'a> {
    pub fn all() -> Self {
        Self::default()
    }

    pub fn from_database(database: Ref<'a>) -> Self {
        Self {
            databases: Some(Expr::from(database)),
        }
    }
}
