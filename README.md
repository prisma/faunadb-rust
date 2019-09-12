# FaunaDB Rust Client

FaunaDB offers an asynchronous client for communicating with
the [Fauna](https://fauna.com) database.

Goals:

- Typesafe
- Allocating only when really needed
- Asynchronous, async/await

The crate is not yet tested on production so use at your own risk.

## Basic Usage

``` rust
use faunadb::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), faunadb::error::Error> {
    let client = Client::builder("put-your-secret-here").build()?;
    let params = DatabaseParams::new("my-first-database");

    let response = client.query(CreateDatabase::new(params)).await?;
    let res = response.resource;

    assert_eq!(Some("my-first-database"), res["name"].as_str())
    Ok(())
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
