use async_trait::{ async_trait };

#[async_trait]
pub trait CommandContext: Send + Sync {
    async fn commit(self: Box<Self>) -> Result<(), anyhow::Error>;
    
    async fn rollback(self: Box<Self>) -> Result<(), anyhow::Error>;
}
