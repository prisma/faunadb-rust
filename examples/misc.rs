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
    let client = ClientBuilder::new(secret).build().unwrap();

    let fun = Filter::new(
        Lambda::new("x", Gt::new(Var::new("x"), 2)),
        Array::from(vec![1, 2, 3]),
    );

    tokio::run(lazy(move || {
        client
            .query(fun)
            .map(|response| {
                println!("{:#?}", response);
            })
            .map_err(|error: faunadb::error::Error| {
                println!("Error: {:#?}", error);
            })
    }));
}
