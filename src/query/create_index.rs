use crate::expr::{Expr, IndexPermission, Object, Ref};
use std::borrow::Cow;

#[derive(Debug, Serialize, Clone)]
pub struct CreateIndex<'a> {
    create_index: IndexParams<'a>,
}

impl<'a> CreateIndex<'a> {
    pub fn new(params: IndexParams<'a>) -> Self {
        Self {
            create_index: params,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Field<'a>(Vec<Cow<'a, str>>);

#[derive(Debug, Serialize, Clone)]
pub struct Binding<'a>(Cow<'a, str>);

#[derive(Debug, Serialize, Clone)]
pub enum TermObject<'a> {
    #[serde(rename = "field")]
    Field(Field<'a>),
    #[serde(rename = "binding")]
    Binding(Binding<'a>),
}

#[derive(Debug, Serialize, Clone)]
pub struct Term<'a> {
    object: TermObject<'a>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ValueObject<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<Field<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binding: Option<Binding<'a>>,
    reverse: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct Value<'a> {
    object: ValueObject<'a>,
}

#[derive(Debug, Serialize, Clone)]
pub struct IndexParamsInternal<'a> {
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

#[derive(Debug, Serialize, Clone)]
pub struct IndexParams<'a> {
    object: IndexParamsInternal<'a>,
}

impl<'a> Term<'a> {
    pub fn field<T>(path: Vec<T>) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        let field = Field(path.into_iter().map(Into::into).collect());

        Self {
            object: TermObject::Field(field),
        }
    }

    pub fn binding<T>(name: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        let binding = Binding(name.into());

        Self {
            object: TermObject::Binding(binding),
        }
    }
}

impl<'a> Value<'a> {
    pub fn field<T>(path: Vec<T>) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        let field = Field(path.into_iter().map(Into::into).collect());

        Self {
            object: ValueObject {
                field: Some(field),
                binding: None,
                reverse: false,
            },
        }
    }

    pub fn binding<T>(name: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        let binding = Binding(name.into());

        Self {
            object: ValueObject {
                field: None,
                binding: Some(binding),
                reverse: false,
            },
        }
    }

    pub fn reverse(&mut self) -> &mut Self {
        self.object.reverse = true;
        self
    }
}

impl<'a> IndexParams<'a> {
    pub fn new<S>(name: S, source: Ref<'a>) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            object: IndexParamsInternal {
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
            },
        }
    }

    pub fn active(&mut self) -> &mut Self {
        self.object.active = true;
        self
    }

    pub fn unique(&mut self) -> &mut Self {
        self.object.unique = true;
        self
    }

    pub fn serialized(&mut self) -> &mut Self {
        self.object.serialized = true;
        self
    }

    pub fn terms(&mut self, terms: Vec<Term<'a>>) -> &mut Self {
        self.object.terms = Some(terms);
        self
    }

    pub fn values(&mut self, values: Vec<Value<'a>>) -> &mut Self {
        self.object.values = Some(values);
        self
    }

    pub fn partitions(&mut self, partitions: u16) -> &mut Self {
        self.object.partitions = Some(partitions);
        self
    }

    pub fn permissions(&mut self, permissions: IndexPermission<'a>) -> &mut Self {
        self.object.permissions = Some(permissions);
        self
    }

    pub fn data(&mut self, data: Object<'a>) -> &mut Self {
        self.object.data = Some(Expr::from(data));
        self
    }
}
