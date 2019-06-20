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
    let client = Client::builder("put-your-secret-here").build().unwrap();
    let params = DatabaseParams::new("my-first-database");

    tokio::run(lazy(move || {
        client
            .query(CreateDatabase::new(params))
            .map(|response| {
                let res = response.resource;
                assert_eq!(Some("my-first-database"), res["name"].as_str())
            })
            .map_err(|error: faunadb::error::Error| {
                println!("Error: {:?}", error);
            })
    }));
}
```

## Testing

For tests to be successful, one must have the [default Fauna Docker
image](https://github.com/fauna/faunadb-docker), using the default password
`secret`.

Run the tests with:

``` bash
cargo test
```

## License

The faunadb-rust crate is licensed under the [Apache 2.0](./LICENSE)
