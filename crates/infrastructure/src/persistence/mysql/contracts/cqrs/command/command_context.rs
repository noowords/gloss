use std::any::{ Any };
use async_trait::{ async_trait };
use sqlx::{ Transaction, MySql };

use application::contracts::cqrs::command::{ CommandContext };

pub struct MySqlCommandContext {
    pub tx: Transaction<'static, MySql>
}

impl MySqlCommandContext {
    pub fn new(tx: Transaction<'static, MySql>) -> Self {
        Self { tx }
    }

    pub fn tx_mut(&mut self) -> &mut Transaction<'static, MySql> {
        &mut self.tx
    }
}

#[async_trait]
impl CommandContext for MySqlCommandContext {
    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }

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
