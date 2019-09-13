use crate::client::Value;
use failure::{self, Fail};
use async_std::future::TimeoutError;

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
    #[fail(display = "Fauna error: {}", _0)]
    DatabaseError(String),
    #[fail(display = "Couldn't convert data: {}", _0)]
    ConversionError(&'static str),
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
        Self::ConnectionError(e.into())
    }
}

impl From<http::uri::InvalidUri> for Error {
    fn from(e: http::uri::InvalidUri) -> Self {
        Self::ConfigurationError(e.into())
    }
}

impl From<hyper::error::Error> for Error {
    fn from(e: hyper::error::Error) -> Self {
        Self::ConnectionError(e.into())
    }
}

impl From<TimeoutError> for Error {
    fn from(_: TimeoutError) -> Self {
        Self::TimeoutError
    }
}
