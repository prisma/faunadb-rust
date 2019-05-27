use crate::error::Error;
use futures::{Future, Poll};
use std::fmt;

pub struct FutureResponse<T>(pub Box<Future<Item = T, Error = Error> + Send + 'static>);

impl<T> Future for FutureResponse<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}
