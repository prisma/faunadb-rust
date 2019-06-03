use std::{borrow::Cow, fmt};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum RefLocation<'a> {
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
    fn path(&self) -> String {
        match self {
            RefLocation::Class { location } => location.path(),
            RefLocation::Index { location } => location.path(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Denotes a resource ref.
pub struct Ref<'a> {
    id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    location: Option<RefLocation<'a>>,
}

impl<'a> fmt::Display for Ref<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.location {
            Some(RefLocation::Class { ref location }) => {
                write!(f, "Ref(id={},class={})", self.id, location.path())
            }
            Some(RefLocation::Index { ref location }) => {
                write!(f, "Ref(id={},index={})", self.id, location.path())
            }
            None => write!(f, "Ref(id={})", self.id),
        }
    }
}

impl<'a> Ref<'a> {
    /// A ref to a singleton instance.
    pub fn instance<S>(id: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            id: id.into(),
            location: None,
        }
    }

    /// A ref to a class.
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

    /// A ref to an index.
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

    /// Set the class for the singleton ref.
    pub fn set_class<S>(&mut self, id: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.location = Some(RefLocation::Class {
            location: Box::new(Self::class(id)),
        });

        self
    }

    /// Set the index for the singleton ref.
    pub fn set_index<S>(&mut self, id: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.location = Some(RefLocation::Index {
            location: Box::new(Self::index(id)),
        });

        self
    }

    /// Gets the fully qualified path.
    pub fn path(&self) -> String {
        match self.location {
            Some(ref location) => format!("{}/{}", location.path(), self.id),
            None => format!("{}", self.id),
        }
    }
}
