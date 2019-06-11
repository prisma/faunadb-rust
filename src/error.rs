use crate::client::Value;
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
    #[fail(display = "Bad request: {}", _0)]
    BadRequest(FaunaErrors),
    #[fail(display = "Not found: {}", _0)]
    NotFound(FaunaErrors),
    #[fail(display = "Request data failure: {}", _0)]
    RequestDataFailure(&'static str),
    #[fail(display = "Response data failure: {}", _0)]
    ResponseDataFailure(&'static str),
    #[fail(display = "Temporary error wrapper for development, original: {}", _0)]
    TemporaryFailure(String),
    #[cfg(feature = "sync_client")]
    #[fail(display = "IO Error: {}", _0)]
    IoError(failure::Error),
}

#[derive(Debug, Deserialize, Fail)]
#[fail(display = "Errors in the request data: [{:?}]", errors)]
pub struct FaunaErrors {
    pub errors: Vec<FaunaError>,
}

#[derive(Debug, Deserialize, Fail)]
#[fail(
    display = "{{position={:?},code={},description={}}}",
    position, code, description
)]
pub struct FaunaError {
    pub position: Vec<Value>,
    pub code: String,
    pub description: String,
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

#[cfg(feature = "sync_client")]
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e.into())
    }
}
