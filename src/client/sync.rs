use super::{Client, Response};
use crate::{expr::Expr, FaunaResult};
use std::sync::Mutex;
use tokio::runtime::Runtime;

/// A synchronous wrapper for the asynchronous Fauna client.
pub struct SyncClient {
    inner: Client,
    runtime: Mutex<Runtime>,
}

impl SyncClient {
    pub fn new(inner: Client) -> FaunaResult<Self> {
        Ok(Self {
            inner,
            runtime: Mutex::new(Runtime::new()?),
        })
    }

    pub fn query<'a, Q>(&self, query: Q) -> FaunaResult<Response>
    where
        Q: Into<Expr<'a>>,
    {
        self.runtime
            .lock()
            .unwrap()
            .block_on(self.inner.query(query))
    }
}
