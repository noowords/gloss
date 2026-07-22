use async_trait::{ async_trait };

use crate::{ CommandContext };

#[async_trait]
pub trait CommandProvider: Send + Sync {
    async fn provide_context(&self) -> Result<Box<dyn CommandContext>, anyhow::Error>;
}
