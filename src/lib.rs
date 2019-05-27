#[macro_use]
extern crate serde_derive;

mod client;
mod error;
mod expr;
mod query;

pub use client::*;
pub use error::*;
pub use expr::*;
pub use query::*;

pub type FaunaResult<T> = Result<T, Error>;
