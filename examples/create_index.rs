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

    let mut params = IndexParams::new("meows", Ref::class("HouseCats"));
    params.permissions(permission);

    let age_term = Term::new(vec!["cats", "age"], "cats_age");
    let name_term = Term::new(vec!["cats", "name"], "cats_name");

    params.terms(vec![age_term, name_term]);

    let name_value = Value::new(vec!["cats", "name"], "cats_name");
    let mut age_value = Value::new(vec!["cats", "age"], "cats_age");

    age_value.reverse();
    params.values(vec![age_value, name_value]);

    tokio::run(lazy(move || {
        client
            .query(CreateIndex::new(params))
            .map(|response| {
                println!("{}", response);
            })
            .map_err(|error: faunadb::error::Error| {
                println!("Error: {:#?}", error);
            })
    }));
}
