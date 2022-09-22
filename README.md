# FaunaDB Rust Client

FaunaDB offers an asynchronous (and synchronous) client for communicating with
the [Fauna](https://fauna.com) database.

Goals:

- Typesafe
- Allocating only when really needed
- Asynchronous using futures (and [Tokio](https://tokio.rs))

The crate is not yet tested on production so use at your own risk.

## SETUP LOCAL RUST DEV ENV
1. Install latest Rust via source & source Cargo env.
```bash
$ cd /tmp
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rust.sh
$ vim rust.sh
$ chmod u+x rust.sh
$ ./rust.sh
$ source $HOME/.cargo/env
```

2. Confirm Rust version & that we're up-to-date.
```bash
$ rustc --version
$ rustup self update
info: checking for self-updates
info: downloading self-update
  rustup updated - 1.25.1 (from 1.24.3)
```

3. Let's follow Rust Nightly & install Rust's nightly toolchain.
``bash
$ rustup default nightly
$ rustup toolchain install nightly
```

## CHANGES MADE SINCE FORKING
1. README updates.
2. Update the following Cargo packages to their latest.
   1. base64-serde -> 0.6.1
   2. base64 -> 0.13.0
   3. clap -> 3
   4. pretty_env_logger -> 0.4
   5. rand -> 0.8
   6. lazy_static -> 1.4

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
