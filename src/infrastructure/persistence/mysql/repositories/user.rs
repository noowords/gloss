use async_trait::{ async_trait };

use crate::domain::common::{ TxContext };
use crate::domain::models::user::{
    User, UserRepository,
    value_objects::{ UserId }
};

use super::super::{
    common::{ MySqlTxContext },
    models::{ MySqlUserRow }
};

#[derive(Default)]
pub struct MySqlUserRepository;

impl MySqlUserRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl UserRepository for MySqlUserRepository {
    async fn create(
        &self,
        ctx: &mut dyn TxContext,
        user: &User
    ) -> Result<(), anyhow::Error> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| anyhow::anyhow!("Invalid TxContext context".to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO users (id, phone, role, created_at)
            VALUES (?, ?, ?, CURRENT_TIMESTAMP)
            "#
        )
            .bind(user.id().value())
            .bind(user.phone().as_ref().map(|p| p.value().clone()))
            .bind(user.role().as_str())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
    
    async fn get_by_id(
        &self,
        ctx: &mut dyn TxContext,
        id: UserId
    ) -> Result<Option<User>, anyhow::Error> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| anyhow::anyhow!("Invalid TxContext context".to_string()))?;

        let row: Option<MySqlUserRow> = sqlx::query_as(
            r#"
            SELECT id, role, phone
            FROM users
            WHERE id = ?
            "#
        )
            .bind(id.value())
            .fetch_optional(&mut *ctx.tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        match row {
            Some(row) => Ok(Some(User::try_from(row)?)),
            None => Ok(None)
        }
    }
    
    async fn exists(
        &self,
        ctx: &mut dyn TxContext,
        id: UserId
    ) -> Result<bool, anyhow::Error> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| anyhow::anyhow!("Invalid TxContext context".to_string()))?;

        let row = sqlx::query(
            r#"
            SELECT 1
            FROM users
            WHERE id = ?
            LIMIT 1
            "#
        )
            .bind(id.value())
            .fetch_optional(&mut *ctx.tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        
        Ok(row.is_some())
    }

    async fn update(
        &self,
        ctx: &mut dyn TxContext,
        user: &User
    ) -> Result<(), anyhow::Error> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| anyhow::anyhow!("Invalid TxContext context".to_string()))?;

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
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
    
    async fn remove(
        &self,
        ctx: &mut dyn TxContext,
        id: UserId
    ) -> Result<(), anyhow::Error> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| anyhow::anyhow!("Invalid TxContext context".to_string()))?;

        sqlx::query(
            r#"
            DELETE FROM users
            WHERE id = ?
            "#
        )
            .bind(id.value())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        
        Ok(())
    }
}
