use async_trait::async_trait;

use crate::KvxError;


#[async_trait(?Send)]
pub trait Execute<O>: Send + Sync {

    type Output: Send;


    async fn execute(
        &self,
        operation: O,
    )
    -> Result<Self::Output, KvxError>;

}