# FaunaDB Rust Client

FaunaDB offers an asynchronous (and synchronous) client for communicating with
the [Fauna](https://fauna.com) database.

Goals:

- Typesafe
- Allocating only when really needed
- Asynchronous using futures (and [Tokio](https://tokio.rs))

The crate is not yet tested on production so use at your own risk.
