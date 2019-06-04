use crate::expr::{Expr, Ref};

#[derive(Debug, Serialize, Clone)]
pub struct Create<'a> {
    #[serde(flatten)]
    reference: Expr<'a>,
    #[serde(skip_serializing)]
    pub(crate) params: InstanceParams<'a>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct InstanceData<'a> {
    data: Expr<'a>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct InstanceParams<'a> {
    object: InstanceData<'a>,
}

impl<'a> Create<'a> {
    pub fn new(class_ref: Ref<'a>, params: InstanceParams<'a>) -> Self {
        Self {
            reference: Expr::from(class_ref),
            params,
        }
    }
}

impl<'a> InstanceParams<'a> {
    pub fn new<E>(data: E) -> Self
    where
        E: Into<Expr<'a>>,
    {
        Self {
            object: InstanceData { data: data.into() },
        }
    }
}
