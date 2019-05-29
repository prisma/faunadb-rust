use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefLocation<'a> {
    #[serde(rename = "class")]
    Class {
        #[serde(rename = "@ref")]
        location: Box<Ref<'a>>,
    },
    #[serde(rename = "index")]
    Index {
        #[serde(rename = "@ref")]
        location: Box<Ref<'a>>,
    },
}

impl<'a> RefLocation<'a> {
    pub fn path(&self) -> String {
        match self {
            RefLocation::Class { location } => location.path(),
            RefLocation::Index { location } => location.path(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ref<'a> {
    id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    location: Option<RefLocation<'a>>,
}

impl<'a> Ref<'a> {
    pub fn instance<S>(id: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            id: id.into(),
            location: None,
        }
    }

    pub fn class<S>(id: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            id: id.into(),
            location: Some(RefLocation::Class {
                location: Box::new(Self::instance("classes")),
            }),
        }
    }

    pub fn index<S>(id: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            id: id.into(),
            location: Some(RefLocation::Index {
                location: Box::new(Self::instance("indexes")),
            }),
        }
    }

    pub fn set_class<S>(&mut self, id: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.location = Some(RefLocation::Class {
            location: Box::new(Self::class(id)),
        });

        self
    }

    pub fn set_index<S>(&mut self, id: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.location = Some(RefLocation::Index {
            location: Box::new(Self::index(id)),
        });

        self
    }

    pub fn path(&self) -> String {
        match self.location {
            Some(ref location) => format!("{}/{}", location.path(), self.id),
            None => format!("{}", self.id),
        }
    }
}
