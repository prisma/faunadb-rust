use crate::expr::Ref;

#[derive(Debug, Serialize)]
pub struct Class<'a> {
    #[serde(rename = "@ref")]
    pub(crate) ref_: Ref<'a>,
    #[serde(skip_serializing)]
    pub(crate) id: &'a str,
}

impl<'a> Class<'a> {
    pub fn new(id: &'a str) -> Self {
        Self {
            ref_: Ref::new("classes"),
            id,
        }
    }

    pub fn path(&self) -> String {
        format!("{}/{}", self.ref_.path(), self.id)
    }
}
