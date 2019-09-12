use crate::prelude::*;
use lazy_static::lazy_static;
use rand::distributions::Alphanumeric;
use rand::Rng;

lazy_static! {
    pub static ref CLIENT: Client = {
        let mut builder = Client::builder("secret");
        builder.uri("http://localhost:8443");

        builder.build().unwrap()
    };
}

pub fn gen_db_name() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect()
}

pub async fn create_database() -> String {
    let db_name = gen_db_name();
    let params = DatabaseParams::new(&db_name);
    CLIENT.query(CreateDatabase::new(params)).await.unwrap();

    db_name
}

pub async fn delete_database(db_name: &str) {
    CLIENT.query(Delete::new(Ref::database(db_name))).await.unwrap();
}

pub async fn create_class() -> String {
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

    CLIENT.query(CreateClass::new(params)).await.unwrap();

    class_name
}

pub async fn delete_class(class_name: &str) {
    CLIENT.query(Delete::new(Ref::class(class_name))).await.unwrap();
}
