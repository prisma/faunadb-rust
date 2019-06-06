use clap::{App, Arg};
use faunadb::{
    prelude::*,
    query::{
        basic::{Lambda, Var},
        collection::Map,
    },
};
use futures::{lazy, Future};

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

    let map = Map::new(
        Map::new(
            Array::from(vec!["Musti", "Naukio"]),
            Lambda::new("cat", Var::new("cat")),
        ),
        Lambda::new("cat", Var::new("cat")),
    );

    tokio::run(lazy(move || {
        client
            .query(map)
            .map(|response| {
                println!("{}", response);
            })
            .map_err(|error: faunadb::error::Error| {
                println!("Error: {:#?}", error);
            })
    }));
}
