use crate::expr::{Expr, Ref};

#[derive(Debug, Serialize, Clone)]
struct CreateInfo<'a>(Expr<'a>);

#[derive(Debug, Serialize, Clone)]
pub struct Create<'a> {
    create: CreateInfo<'a>,
    params: InstanceParams<'a>,
}

#[derive(Debug, Serialize, Clone)]
pub struct InstanceData<'a> {
    data: Expr<'a>,
}

#[derive(Debug, Serialize, Clone)]
pub struct InstanceParams<'a> {
    object: InstanceData<'a>,
}

impl<'a> Create<'a> {
    pub fn new(class_ref: Ref<'a>, params: InstanceParams<'a>) -> Self {
        Self {
            create: CreateInfo(Expr::from(class_ref)),
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
