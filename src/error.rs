use failure::{self, Fail};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Error connecting to the database: {}", _0)]
    ConnectionError(failure::Error),
    #[fail(display = "Configuration error: {}", _0)]
    ConfigurationError(failure::Error),
    #[fail(display = "Timed out")]
    TimeoutError,
    #[fail(display = "Unknown error")]
    Other,
    #[fail(display = "Unauthorized")]
    Unauthorized,
    #[fail(display = "Server sent no response")]
    EmptyResponse,
    #[fail(display = "Temporary error wrapper for development, original: {}", _0)]
    TemporaryFailure(String),
}

impl From<native_tls::Error> for Error {
    fn from(e: native_tls::Error) -> Self {
        Error::ConnectionError(e.into())
    }
}

impl From<http::uri::InvalidUri> for Error {
    fn from(e: http::uri::InvalidUri) -> Self {
        Error::ConfigurationError(e.into())
    }
}
