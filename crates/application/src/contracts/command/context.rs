use std::any::{ Any };
use async_trait::{ async_trait };

#[async_trait]
pub trait CommandContext: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    
    fn as_any_mut(&mut self) -> &mut dyn Any;

    async fn commit(self: Box<Self>) -> Result<(), anyhow::Error>;
    
    async fn rollback(self: Box<Self>) -> Result<(), anyhow::Error>;
}
