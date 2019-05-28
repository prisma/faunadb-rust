#[derive(Debug)]
pub enum Ref<'a> {
    Instance(&'a str, Box<Ref<'a>>),
    Class(&'a str),
    Index(&'a str),
}

impl<'a> Ref<'a> {
    pub fn new(id: &'a str, location: Ref<'a>) -> Self {
        Ref::Instance(id, Box::new(location))
    }

    pub fn class(id: &'a str) -> Self {
        Ref::Class(id)
    }

    pub fn index(id: &'a str) -> Self {
        Ref::Index(id)
    }

    pub fn path(&self) -> String {
        match self {
            Ref::Instance(id, location) => format!("{}/{}", location.path(), id),
            Ref::Class(id) => format!("classes/{}", id),
            Ref::Index(id) => format!("indexes/{}", id),
        }
    }
}
