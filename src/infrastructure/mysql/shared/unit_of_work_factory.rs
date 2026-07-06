use std::sync::{ Arc };
use async_trait::{ async_trait };

use crate::domain::shared::{ PoolContext, UnitOfWork, UnitOfWorkFactory };

use super::{ MySqlPoolContext, MySqlUnitOfWork };

pub struct MySqlUnitOfWorkFactory {
    cpool: Arc<dyn PoolContext>
}

impl MySqlUnitOfWorkFactory {
    pub fn new(cpool: Arc<dyn PoolContext>) -> Self {
        Self { cpool }
    }
}

#[async_trait]
impl UnitOfWorkFactory for MySqlUnitOfWorkFactory {
    async fn begin(&self) -> Result<Box<dyn UnitOfWork>, anyhow::Error> {
        let ctx = self.cpool
            .downcast_ref::<MySqlPoolContext>()
            .ok_or_else(|| anyhow::anyhow!("Expected MySqlPoolContext inside MySqlUnitOfWorkFactory"))?;

        let uow = MySqlUnitOfWork::begin(&ctx.pool).await?;
        
        Ok(Box::new(uow))
    }
}
