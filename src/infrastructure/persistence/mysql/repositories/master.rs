use std::cell::{ UnsafeCell };
use async_trait::{ async_trait };
use sqlx::{ Transaction, MySql };
use serde_json;

use crate::domain::models::{
    user::value_objects::{ UserId },
    master::{ Master, MasterRepository }
};

use super::super::models::{ MySqlMasterRow };

pub struct MySqlMasterRepository<'a> {
    tx: UnsafeCell<&'a mut Transaction<'static, MySql>>
}

unsafe impl<'a> Send for MySqlMasterRepository<'a> {}
unsafe impl<'a> Sync for MySqlMasterRepository<'a> {}

impl<'a> MySqlMasterRepository<'a> {
    pub fn new(tx: &'a mut Transaction<'static, MySql>) -> Self {
        Self { tx: UnsafeCell::new(tx) }
    }

    #[inline(always)]
    fn tx_mut(&self) -> &mut sqlx::MySqlConnection {
        unsafe {
            let tx_ref = &mut *self.tx.get();
            tx_ref.as_mut()
        }
    }
}

#[async_trait]
impl<'a> MasterRepository for MySqlMasterRepository<'a> {
    async fn create(&self, master: &Master) -> Result<(), anyhow::Error> {
        let schedule_json = serde_json::to_value(master.schedule())
            .map_err(|e| anyhow::anyhow!("MasterSchedule corrupted: {}", e.to_string()))?;
            
        sqlx::query(
            r#"
            INSERT INTO masters (user_id, schedule)
            VALUES (?, ?)
            "#
        )
            .bind(master.user_id().value())
            .bind(&schedule_json)
            .execute(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
    
    async fn get_by_user_id(&self, user_id: UserId) -> Result<Option<Master>, anyhow::Error> {
        let row: Option<MySqlMasterRow> = sqlx::query_as(
            r#"
            SELECT user_id, schedule
            FROM masters
            WHERE user_id = ?
            "#
        )
            .bind(user_id.value())
            .fetch_optional(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        match row {
            Some(row) => Ok(Some(Master::try_from(row)?)),
            None => Ok(None)
        }
    }
    
    async fn exists(&self, user_id: UserId) -> Result<bool, anyhow::Error> {
        let row = sqlx::query(
            r#"
            SELECT 1
            FROM masters
            WHERE user_id = ?
            LIMIT 1
            "#
        )
            .bind(user_id.value())
            .fetch_optional(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        
        Ok(row.is_some())
    }

    async fn update(&self, master: &Master) -> Result<(), anyhow::Error> {
        let row = MySqlMasterRow::from(master);

        sqlx::query(
            r#"
            UPDATE masters SET schedule = ?
            WHERE user_id = ?
            "#
        )
            .bind(row.schedule())
            .bind(row.user_id())
            .execute(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
    
    async fn remove(&self, user_id: UserId) -> Result<(), anyhow::Error> {
        sqlx::query(
            r#"
            DELETE FROM masters
            WHERE user_id = ?
            "#
        )
            .bind(user_id.value())
            .execute(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        
        Ok(())
    }
}
