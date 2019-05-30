use clap::{App, Arg};
use faunadb::prelude::*;
use futures::{future::lazy, Future};

fn main() {
    pretty_env_logger::init();

    let matches = App::new("A Simple FaunaDB Client")
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
    let client = ClientBuilder::new(secret).build().unwrap();

    let mut params = Object::new();
    params.insert("name", "ellu");
    params.insert("age", 10);

    let mut data = Object::new();
    data.insert("data", params);

    tokio::run(lazy(move || {
        client
            .query(Create::instance(Ref::class("HouseCats"), data))
            .map(|response| {
                println!("{}", response);
            })
            .map_err(|error: faunadb::error::Error| {
                println!("Error: {:#?}", error);
            })
    }));
}
