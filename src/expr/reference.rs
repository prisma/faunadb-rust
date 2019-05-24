use crate::expr::{Class, Index};

#[derive(Debug, Serialize)]
pub struct Ref<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    class: Option<Box<Class<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<Box<Index<'a>>>,
    id: &'a str,
}

impl<'a> Ref<'a> {
    pub fn new(id: &'a str) -> Self {
        Self {
            id,
            class: None,
            index: None,
        }
    }

    pub fn class(id: &'a str, class: Class<'a>) -> Self {
        Self {
            id,
            class: Some(Box::new(class)),
            index: None,
        }
    }

    pub fn index(index: Index<'a>) -> Self {
        Self {
            id: "",
            class: None,
            index: Some(Box::new(index)),
        }
    }

    pub fn path(&self) -> String {
        match self.class {
            Some(ref class) => format!("{}/{}", class.path(), self.id),
            None => match self.index {
                Some(ref index) => format!("{}", index.path()),
                None => String::from(self.id),
            },
        }
    }
}
