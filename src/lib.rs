//! # FaunaDB
//!
//! `faunadb` is a client for the Fauna database in Rust. It provides the query
//! and expression types, (de-)serialization and an asynchronous client.
//!
//! Additionally the crate holds a `SyncClient` wrapper for synchronous
//! execution, enabled with the `sync_client` feature flag.
//!
//! Most of the type checks are handled in Fauna and the functions accept
//! anything that can be converted to the [Expr](expr/struct.Expr.html) enum,
//! allowing the usage of different Fauna types in a more dynamic manner.
//!
//! ## Asynchronous example:
//!
//! ```no_run
//! use futures::{future::lazy, Future};
//! use faunadb::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> std::result::Result<(), faunadb::error::Error> {
//!     let client = Client::builder("my_fauna_secret").build()?;
//!
//!     let query = Filter::new(
//!         Lambda::new("x", Gt::new(Var::new("x"), 2)),
//!         Array::from(vec![1, 2, 3]),
//!     );
//!
//!     let response = client.query(query).await?;
//!     println!("{:#?}", response);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Synchronous example:
//!
//! ```no_run
//! use faunadb::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> std::result::Result<(), faunadb::error::Error> {
//!     let mut client = Client::builder("my_fauna_secret").build()?;
//!
//!     let query = Filter::new(
//!         Lambda::new("x", Gt::new(Var::new("x"), 2)),
//!         Array::from(vec![1, 2, 3]),
//!     );
//!
//!     let response = client.query(query).await?;
//!     println!("{:#?}", response);
//!
//!     Ok(())
//! }
//! ```
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

#[macro_use]
mod macros;

pub mod client;
pub mod error;
pub mod expr;
pub mod prelude;
pub mod query;

mod serde;

#[cfg(test)]
mod test_utils;

pub type Result<T> = ::std::result::Result<T, error::Error>;
