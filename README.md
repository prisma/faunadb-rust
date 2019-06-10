# FaunaDB Rust Client

FaunaDB offers an asynchronous (and synchronous) client for communicating with
the [Fauna](https://fauna.com) database.

Goals:

- Typesafe
- Allocating only when really needed
- Asynchronous using futures (and [Tokio](https://tokio.rs))

The crate is not yet tested on production so use at your own risk.

## Basic Usage

``` rust
use faunadb::prelude::*;
use tokio;
use futures::{future::lazy, Future};

fn main() {
    let client = ClientBuilder::new("put-your-secret-here").build();
    let params = DatabaseParams::new("my-first-database");

    tokio::run(lazy(move || {
        client
            .query(CreateDatabase::new(params))
            .map(|response| {
                println!("{}", response);
            })
            .map_err(|error: faunadb::error::Error| {
                println!("Error: {:?}", error);
            })
    }));
}
```

## License

The faunadb-rust crate is licensed under the [Apache 2.0](./LICENSE)
