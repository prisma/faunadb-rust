use crate::prelude::*;
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

pub fn with_class<F>(f: F)
where
    F: FnOnce(&str) -> () + panic::UnwindSafe,
{
    let mut permission = ClassPermission::default();
    permission.read(Level::public());
    permission.write(Level::public());

    let mut data = Object::default();
    data.insert("meow", true);

    let class_name = gen_db_name();

    let mut params = ClassParams::new(&class_name);
    params.history_days(10);
    params.ttl_days(3);
    params.permissions(permission);
    params.data(data);

    with_database(|_| {
        trace!("Creating a test class {}", &class_name);
        CLIENT.query(CreateClass::new(params)).unwrap();

        f(class_name.as_str());

        trace!("Creating the test class {}", &class_name);
        CLIENT.query(Delete::new(Ref::class(&class_name))).unwrap();
    })
}
