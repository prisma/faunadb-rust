use crate::expr::Ref;

#[derive(Debug, Serialize)]
pub struct Index<'a> {
    #[serde(rename = "@ref")]
    pub(crate) ref_: Ref<'a>,
    #[serde(skip_serializing)]
    pub(crate) id: &'a str,
}

impl<'a> Index<'a> {
    pub fn new(id: &'a str) -> Self {
        Self {
            ref_: Ref::new("indexes"),
            id,
        }
    }

    pub fn path(&self) -> String {
        format!("{}/{}", self.ref_.path(), self.id)
    }
}
