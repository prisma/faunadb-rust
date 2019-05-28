use crate::expr::Expr;

#[derive(Debug, Serialize, Deserialize)]
pub enum Ref<'a> {
    ClassInstance { id: &'a str, class: Box<Expr<'a>> },
    IndexInstance { id: &'a str, index: Box<Expr<'a>> },
    Class { id: &'a str },
    Index { id: &'a str },
}

impl<'a> Ref<'a> {
    pub fn class_instance(id: &'a str, location: Ref<'a>) -> Self {
        let class = Box::new(Expr::from(location));
        Ref::ClassInstance { id, class }
    }

    pub fn index_instance(id: &'a str, location: Ref<'a>) -> Self {
        let index = Box::new(Expr::from(location));
        Ref::IndexInstance { id, index }
    }

    pub fn class(id: &'a str) -> Self {
        Ref::Class { id }
    }

    pub fn index(id: &'a str) -> Self {
        Ref::Index { id }
    }
}
