use crate::Error;
use futures::Future;
use std::fmt;

pub struct FutureResponse<T>(pub Box<Future<Item = T, Error = Error> + Send + 'static>);
