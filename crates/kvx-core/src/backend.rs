use async_trait::async_trait;
use crate::{KvxError, Request, Response};


pub trait BackendFactory {
    type Client;
    type Options;
}

#[async_trait(?Send)]
pub trait Backend: Send + Sync {
    async fn execute(
        &self,
        request: Request,
    ) -> Result<Response, KvxError>;
}


