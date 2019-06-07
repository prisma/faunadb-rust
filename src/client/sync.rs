use super::{Client, Response};
use crate::{expr::Expr, FaunaResult};
use tokio::runtime::Runtime;

/// A synchronous wrapper for the asynchronous Fauna client.
pub struct SyncClient {
    inner: Client,
    runtime: Runtime,
}

impl SyncClient {
    pub fn new(inner: Client) -> FaunaResult<Self> {
        Ok(Self {
            inner,
            runtime: Runtime::new()?,
        })
    }

    pub fn query<'a, Q>(&mut self, query: Q) -> FaunaResult<Response>
    where
        Q: Into<Expr<'a>>,
    {
        self.runtime.block_on(self.inner.query(query))
    }
}
