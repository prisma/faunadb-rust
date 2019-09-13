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

    let params = FunctionParams::new(
        "double",
        Lambda::new(
            "x",
            Add::new(Array::from(vec![Var::new("x"), Var::new("x")])),
        ),
    );

    let response = client.query(CreateFunction::new(params)).await?;
    println!("{:?}", response);

    Ok(())
}
