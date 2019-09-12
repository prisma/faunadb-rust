use clap::{App, Arg};
use faunadb::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), faunadb::error::Error> {
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

    let mut builder = Client::builder(secret);
    builder.uri("http://localhost:8443");

    let client = builder.build().unwrap();
    let mut data = Object::default();
    data.insert("foo", "bar");

    let mut params = DatabaseParams::new("test");
    params.priority(10).unwrap();
    params.data(data);

    let response = client.query(CreateDatabase::new(params)).await?;
    println!("{:#?}", response);

    Ok(())
}
