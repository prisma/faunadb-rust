use chrono::{NaiveDate, Utc};
use clap::{App, Arg};
use faunadb::prelude::*;
use futures::{
    Future,
    {future::Either, lazy},
};

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
            Arg::with_name("create_class")
                .short("c")
                .long("create_class")
                .required(false)
                .help("Create a new class called HouseCats")
                .takes_value(false),
        )
        .get_matches();

    let secret = matches.value_of("secret").unwrap();
    let client = ClientBuilder::new(secret).build().unwrap();

    let create_instance = {
        let mut obj = Object::default();

        obj.insert("name", "Musti");
        obj.insert("id", 1);
        obj.insert("age", 7);
        obj.insert("byte_data", Bytes::from(vec![0x1, 0x2, 0x3]));
        obj.insert(
            "nicknames",
            Array::from(vec!["mustu", "muspus", "mustikka"]),
        );
        obj.insert("this_is_null", Expr::null());
        obj.insert("am_i_cute", true);
        obj.insert("created_at", Utc::now());
        obj.insert("birthday", NaiveDate::from_ymd(2011, 7, 7));

        {
            let mut obj2 = Object::default();
            obj2.insert("foo", "bar");
            obj.insert("objective", obj2);
        }

        let params = InstanceParams::new(obj);

        Create::new(Ref::class("HouseCats"), params)
    };

    let instance_query = client.query(create_instance);

    let query = if matches.is_present("create_class") {
        let mut perms = ClassPermission::default();
        perms.read(Level::public());

        let mut params = ClassParams::new("HouseCats");
        params.history_days(3);
        params.ttl_days(3);
        params.permissions(perms);

        let class_query = client.query(CreateClass::new(params));

        let query = class_query.and_then(|res| {
            println!("{:?}", res);
            instance_query
        });

        Either::A(query)
    } else {
        Either::B(instance_query)
    };

    tokio::run(lazy(move || {
        query
            .map(|response| {
                println!("{:?}", response);
            })
            .map_err(|error: faunadb::error::Error| {
                println!("Error: {:#?}", error);
            })
    }));
}
