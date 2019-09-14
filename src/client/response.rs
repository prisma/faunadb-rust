mod index;
mod value;

use crate::error::Error;
use futures::{Future, Poll};

pub use index::*;
pub use value::*;

pub struct FutureResponse<T>(pub Box<dyn Future<Item = T, Error = Error> + Send + 'static>);

impl<T> Future for FutureResponse<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Response {
    pub resource: Value,
}
