use async_trait::{ async_trait };
use sqlx::{ Transaction, MySql };

use application::{ CommandContext };

pub struct MySqlCommandContext {
    tx: Transaction<'static, MySql>
}

impl MySqlCommandContext {
    pub fn new(tx: Transaction<'static, MySql>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl CommandContext for MySqlCommandContext {
    async fn commit(self: Box<Self>) -> Result<(), anyhow::Error> {
        self.tx.commit()
            .await
            .map_err(|e| anyhow::anyhow!("Transaction commit failed: {}", e.to_string()))
    }

    async fn rollback(self: Box<Self>) -> Result<(), anyhow::Error> {
        self.tx.rollback()
            .await
            .map_err(|e| anyhow::anyhow!("Transaction rollback failed: {}", e.to_string()))
    }
}
