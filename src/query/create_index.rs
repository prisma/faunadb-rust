use crate::expr::{Expr, IndexPermission, Ref, Object};
use std::borrow::Cow;

#[derive(Debug, Serialize)]
pub struct CreateIndex<'a> {
    object: IndexParams<'a>,
}

impl<'a> CreateIndex<'a> {
    pub fn new(params: IndexParams<'a>) -> Self {
        Self { object: params }
    }
}

#[derive(Debug, Serialize)]
pub struct Term<'a> {
    field: Vec<Cow<'a, str>>,
    binding: Cow<'a, str>,
}

impl<'a> Term<'a> {
    pub fn new<T, U>(field: Vec<T>, binding: U) -> Self
    where
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        Self {
            field: field.into_iter().map(Into::into).collect(),
            binding: binding.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Value<'a> {
    field: Vec<Cow<'a, str>>,
    binding: Cow<'a, str>,
    reverse: bool,
}

impl<'a> Value<'a> {
    pub fn new<T, U>(field: Vec<T>, binding: U) -> Self
    where
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        Self {
            field: field.into_iter().map(Into::into).collect(),
            binding: binding.into(),
            reverse: false,
        }
    }

    pub fn reverse(&mut self) -> &mut Self {
        self.reverse = true;
        self
    }
}

#[derive(Debug, Serialize)]
pub struct IndexParams<'a> {
    name: Cow<'a, str>,
    source: Expr<'a>,
    active: bool,
    unique: bool,
    serialized: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    terms: Option<Vec<Term<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<Vec<Value<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partitions: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<IndexPermission<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Expr<'a>>,
}

impl<'a> IndexParams<'a> {
    pub fn new<S>(name: S, source: Ref<'a>) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            name: name.into(),
            source: Expr::from(source),
            active: false,
            unique: false,
            serialized: false,
            terms: None,
            values: None,
            partitions: None,
            permissions: None,
            data: None,
        }
    }

    pub fn active(&mut self) -> &mut Self {
        self.active = true;
        self
    }

    pub fn unique(&mut self) -> &mut Self {
        self.unique = true;
        self
    }

    pub fn serialized(&mut self) -> &mut Self {
        self.serialized = true;
        self
    }

    pub fn terms(&mut self, terms: Vec<Term<'a>>) -> &mut Self {
        self.terms = Some(terms);
        self
    }

    pub fn values(&mut self, values: Vec<Value<'a>>) -> &mut Self {
        self.values = Some(values);
        self
    }

    pub fn partitions(&mut self, partitions: u16) -> &mut Self {
        self.partitions = Some(partitions);
        self
    }

    pub fn permissions(&mut self, permissions: IndexPermission<'a>) -> &mut Self {
        self.permissions = Some(permissions);
        self
    }
    pub fn data(&mut self, data: Object<'a>) -> &mut Self {
        self.data = Some(Expr::from(data));
        self
    }
}
