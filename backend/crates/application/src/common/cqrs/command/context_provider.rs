use async_trait::{ async_trait };

use super::{ CommandContext };

#[async_trait]
pub trait CommandContextProvider: Send + Sync + 'static {
    async fn provide_context(&self) -> Result<Box<dyn CommandContext>, anyhow::Error>;
}
