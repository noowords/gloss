use async_trait::{ async_trait };
use sqlx::mysql::{ MySqlPool };

use application::persistence::contexts::{ TxContext, UnitOfWork };

use super::{ MySqlTxContext };

pub struct MySqlUnitOfWork {
    ctx: MySqlTxContext
}

impl MySqlUnitOfWork {
    pub async fn begin(pool: &MySqlPool) -> Result<Self, anyhow::Error> {
        let tx = pool.begin().await
            .map_err(|e| anyhow::anyhow!("Transaction begin failed: {}", e.to_string()))?;
        
        let ctx = MySqlTxContext::new(tx);
        
        Ok(Self { ctx })
    }
}

#[async_trait]
impl UnitOfWork for MySqlUnitOfWork {
    fn ctx_mut(&mut self) -> &mut dyn TxContext {
        &mut self.ctx
    }

    async fn commit(self: Box<Self>) -> Result<(), anyhow::Error> {
        self.ctx.tx.commit().await
            .map_err(|e| anyhow::anyhow!("Transaction commit failed: {}", e.to_string()))
    }

    async fn rollback(self: Box<Self>) -> Result<(), anyhow::Error> {
        self.ctx.tx.rollback().await
            .map_err(|e| anyhow::anyhow!("Transaction rollback failed: {}", e.to_string()))
    }
}
