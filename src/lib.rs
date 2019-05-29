#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

pub mod client;
pub mod error;
pub mod expr;
pub mod prelude;
pub mod query;

mod serde;

pub type FaunaResult<T> = Result<T, error::Error>;
