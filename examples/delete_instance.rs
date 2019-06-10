use clap::{App, Arg};
use faunadb::{prelude::*, query::write::Delete};
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
        .arg(
            Arg::with_name("id")
                .short("i")
                .long("id")
                .value_name("STRING")
                .required(true)
                .help("ID of the instance")
                .takes_value(true),
        )
        .get_matches();

    let secret = matches.value_of("secret").unwrap();

    let mut builder = ClientBuilder::new(secret);
    builder.uri("http://localhost:8443");
    let client = builder.build().unwrap();

    tokio::run(lazy(move || {
        let instance = Ref::database(matches.value_of("id").unwrap());

        let query = Delete::new(instance);

        client
            .query(query)
            .map(|response| {
                println!("{:?}", response);
            })
            .map_err(|error: faunadb::error::Error| {
                println!("Error: {:?}", error);
            })
    }));
}
