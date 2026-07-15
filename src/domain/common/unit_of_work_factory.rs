use async_trait::{ async_trait };

use crate::domain::common::{ UnitOfWork };

#[async_trait]
pub trait UnitOfWorkFactory: Send + Sync {
    async fn begin(&self) -> Result<Box<dyn UnitOfWork>, anyhow::Error>;
}
