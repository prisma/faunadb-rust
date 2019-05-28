use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum Ref<'a> {
    Instance(Cow<'a, str>, Box<Ref<'a>>),
    Class(Cow<'a, str>),
    Index(Cow<'a, str>),
}

impl<'a> Ref<'a> {
    pub fn new<S>(id: S, location: Ref<'a>) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Ref::Instance(id.into(), Box::new(location))
    }

    pub fn class<S>(id: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Ref::Class(id.into())
    }

    pub fn index<S>(id: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Ref::Index(id.into())
    }

    pub fn path(&self) -> String {
        match self {
            Ref::Instance(id, location) => format!("{}/{}", location.path(), id),
            Ref::Class(id) => format!("classes/{}", id),
            Ref::Index(id) => format!("indexes/{}", id),
        }
    }
}
