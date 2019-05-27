use faunadb::prelude::*;
use futures::{future, Future};
use tokio::runtime::current_thread::Runtime;

fn main() {
    let mut runtime = Runtime::new().unwrap();
    let client = Client::new().unwrap();

    let mut params = Object::new();
    params.insert("test_field", "test_value");

    let mut data = Object::new();
    data.insert("data", params);

    let requesting = client
        .query(Create::instance(Class::new("test"), data))
        .then(|res| {
            println!("GOT A RESULT: {:?}", res);
            future::ok(())
        });

    runtime.spawn(requesting);
    runtime.run().unwrap();
}
