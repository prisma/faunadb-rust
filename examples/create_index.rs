use clap::{App, Arg};
use faunadb::prelude::*;
use futures::{lazy, Future};

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

    let mut permission = IndexPermission::default();
    permission.read(Level::public());

    let mut params = IndexParams::new("new_meows", Ref::class("HouseCats"));
    params.permissions(permission);
    params.serialized();
    params.partitions(8);

    let id_term = Term::field(vec!["data", "id"]);

    params.terms(vec![id_term]);

    let ref_value = Value::field(vec!["ref"]);
    let name_value = Value::field(vec!["data", "name"]);
    let mut age_value = Value::field(vec!["data", "age"]);

    age_value.reverse();
    params.values(vec![ref_value, age_value, name_value]);

    tokio::run(lazy(move || {
        client
            .query(CreateIndex::new(params))
            .map(|response| {
                println!("{:?}", response);
            })
            .map_err(|error: faunadb::error::Error| {
                println!("Error: {:#?}", error);
            })
    }));
}
