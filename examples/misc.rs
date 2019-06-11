use clap::{App, Arg};
use faunadb::prelude::*;
use futures::{future::lazy, Future};

fn main() {
    pretty_env_logger::init();

    let matches = App::new("A misc throwaway test client for development")
        .version("1.0")
        .author("Julius de Bruijn <bruijn@prisma.io>")
        .about("For testing faunadb-rust client library")
        .arg(
            Arg::with_name("secret")
                .short("s")
                .long("secret")
                .value_name("STRING")
                .required(true)
                .help("The FaunaDB connection secret")
                .takes_value(true),
        )
        .get_matches();

    let secret = matches.value_of("secret").unwrap();

    let mut builder = ClientBuilder::new(secret);
    builder.uri("http://localhost:8443");

    let client = builder.build().unwrap();
    let mut data = Object::default();
    data.insert("foo", "bar");

    let mut params = DatabaseParams::new("test");
    params.priority(10).unwrap();
    params.data(data);

    tokio::run(lazy(move || {
        client
            .query(CreateDatabase::new(params))
            .map(|response| {
                println!("{:#?}", response);
            })
            .map_err(|error: faunadb::error::Error| {
                println!("Error: {:#?}", error);
            })
    }));
}
