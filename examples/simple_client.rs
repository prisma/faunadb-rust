use faunadb::prelude::*;
use futures::{
    future::{self, lazy},
    Future,
};

fn main() {
    let client = ClientBuilder::new("SECRET").build().unwrap();

    let mut params = Object::new();
    params.insert("test_field", "test_value");

    let mut data = Object::new();
    data.insert("data", params);

    tokio::run(lazy(move || {
        client
            .query(Create::instance(Class::new("test"), data))
            .map(|response| {
                println!("Success: {:?}", response);
            })
            .map_err(|error: faunadb::error::Error| {
                println!("Error: {:?}", error);
            })
    }));
}
