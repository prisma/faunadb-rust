use crate::{
    client::{ClientBuilder, SyncClient},
    expr::Ref,
    query::write::{CreateDatabase, DatabaseParams, Delete},
};
use lazy_static::lazy_static;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::panic;

lazy_static! {
    pub static ref CLIENT: SyncClient = {
        let mut builder = ClientBuilder::new("secret");
        builder.uri("http://localhost:8443");

        builder.build_sync().unwrap()
    };
}

pub fn gen_db_name() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect()
}

pub fn with_database<F>(f: F)
where
    F: FnOnce(&str) -> () + panic::UnwindSafe,
{
    let db_name = gen_db_name();
    let params = DatabaseParams::new(&db_name);

    trace!("Creating a test database {}", &db_name);
    CLIENT.query(CreateDatabase::new(params)).unwrap();

    let result = panic::catch_unwind(|| f(db_name.as_ref()));

    trace!("Deleting the test database {}", &db_name);
    CLIENT.query(Delete::new(Ref::database(&db_name))).unwrap();

    result.unwrap();
}
