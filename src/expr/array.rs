use super::Expr;
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize)]
pub struct Array<'a>(pub Vec<Expr<'a>>);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bytes<'a>(pub Cow<'a, [u8]>);

impl<'a> Array<'a> {
    pub fn reuse(self) -> Self {
        let reused = self.0.into_iter().map(|e| e.reuse()).collect();
        Array(reused)
    }

    pub fn push(&mut self, e: impl Into<Expr<'a>>) -> &mut Self {
        self.0.push(e.into());
        self
    }
}

impl<'a, E> From<Vec<E>> for Array<'a>
where
    E: Into<Expr<'a>>,
{
    fn from(a: Vec<E>) -> Self {
        Array(a.into_iter().map(Into::into).collect())
    }
}

impl<'a, B> From<B> for Bytes<'a>
where
    B: Into<Cow<'a, [u8]>>,
{
    fn from(b: B) -> Self {
        Self(b.into())
    }
}
