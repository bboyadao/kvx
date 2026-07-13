use async_trait::async_trait;

use crate::KvxError;

#[async_trait(?Send)]
pub trait Handler<O>: Send + Sync {

    type Output: Send;


    async fn handle(
        &self,
        operation: O,
    )
    -> Result<Self::Output, KvxError>;

}