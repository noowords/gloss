use std::cell::{ UnsafeCell };
use async_trait::{ async_trait };
use sqlx::{ Transaction, MySql };

use domain::models::user::{
    User, UserRepository,
    value_objects::{ UserId }
};

use super::super::models::{ MySqlUserRow };

pub struct MySqlUserRepository<'a> {
    tx: UnsafeCell<&'a mut Transaction<'static, MySql>>
}

unsafe impl<'a> Send for MySqlUserRepository<'a> {}
unsafe impl<'a> Sync for MySqlUserRepository<'a> {}

impl<'a> MySqlUserRepository<'a> {
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
impl<'a> UserRepository for MySqlUserRepository<'a> {
    async fn create(&self, user: &User) -> Result<(), anyhow::Error> {
        sqlx::query(
            r#"
            INSERT INTO users (id, phone, role, created_at)
            VALUES (?, ?, ?, CURRENT_TIMESTAMP)
            "#
        )
            .bind(user.id().value())
            .bind(user.phone().as_ref().map(|p| p.value().clone()))
            .bind(user.role().as_str())
            .execute(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
    
    async fn get_by_id(&self, id: UserId) -> Result<Option<User>, anyhow::Error> {
        let row: Option<MySqlUserRow> = sqlx::query_as(
            r#"
            SELECT id, role, phone
            FROM users
            WHERE id = ?
            "#
        )
            .bind(id.value())
            .fetch_optional(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        match row {
            Some(row) => Ok(Some(User::try_from(row)?)),
            None => Ok(None)
        }
    }
    
    async fn exists(&self, id: UserId) -> Result<bool, anyhow::Error> {
        let row = sqlx::query(
            r#"
            SELECT 1
            FROM users
            WHERE id = ?
            LIMIT 1
            "#
        )
            .bind(id.value())
            .fetch_optional(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        
        Ok(row.is_some())
    }

    async fn update(&self, user: &User) -> Result<(), anyhow::Error> {
        let row = MySqlUserRow::from(user);

        sqlx::query(
            r#"
            UPDATE users
            SET phone = ?, role = ?
            WHERE id = ?
            "#
        )
            .bind(row.phone())
            .bind(row.role())
            .bind(row.id())
            .execute(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
    
    async fn remove(&self, id: UserId) -> Result<(), anyhow::Error> {
        sqlx::query(
            r#"
            DELETE FROM users
            WHERE id = ?
            "#
        )
            .bind(id.value())
            .execute(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        
        Ok(())
    }
}
