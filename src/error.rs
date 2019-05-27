use failure::{self, Fail};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Error connecting to the database: {}", _0)]
    ConnectionError(failure::Error),
}

impl From<native_tls::Error> for Error {
    fn from(e: native_tls::Error) -> Self {
        Error::ConnectionError(e.into())
    }
}
