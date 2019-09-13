use clap::{App, Arg};
use faunadb::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), faunadb::error::Error> {
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
    let client = Client::builder(secret).build().unwrap();

    let mut permission = IndexPermission::default();
    permission.read(Level::public());

    let mut params = IndexParams::new("new_meows", Ref::class("HouseCats"));
    params.permissions(permission);
    params.serialized();
    params.partitions(8);

    let id_term = Term::field(vec!["data", "id"]);

    params.terms(vec![id_term]);

    let ref_value = IndexValue::field(vec!["ref"]);
    let name_value = IndexValue::field(vec!["data", "name"]);
    let mut age_value = IndexValue::field(vec!["data", "age"]);

    age_value.reverse();
    params.values(vec![ref_value, age_value, name_value]);

    let response = client.query(CreateIndex::new(params)).await?;
    println!("{:?}", response);

    Ok(())
}
