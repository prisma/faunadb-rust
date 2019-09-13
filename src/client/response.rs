mod index;
mod value;

pub use index::*;
pub use value::*;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Response {
    pub resource: Value,
}
