pub mod driver;
pub mod error;
pub mod execute_ext;
pub mod handler;

pub use driver::Driver;
pub use error::KvError;
pub use execute_ext::ExecuteExt;
pub use handler::Handler;

use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait Connection: Send + Sync {

    async fn get(
        &self,
        key: &str,
    ) -> Result<Option<Vec<u8>>>;


    async fn set(
        &self,
        key: &str,
        value: &[u8],
    ) -> Result<()>;

    async fn delete(
        &self,
        key: &str,
    ) -> Result<()>;
}