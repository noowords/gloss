use async_trait::{ async_trait };

use crate::domain::models::{
    user::value_objects::{ UserId },
    profile::{ Profile, ProfileRepository }
};
use crate::application::common::persistence::{ TxContext };

use super::super::{
    common::{ MySqlTxContext },
    models::{ MySqlProfileRow }
};

#[derive(Default)]
pub struct MySqlProfileRepository;

impl MySqlProfileRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl ProfileRepository for MySqlProfileRepository {
    async fn create(
        &self,
        ctx: &mut dyn TxContext,
        profile: &Profile
    ) -> Result<(), anyhow::Error> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| anyhow::anyhow!("Invalid TxContext context".to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO profiles (user_id, first_name, last_name, avatar_url, bio)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
            .bind(profile.user_id().value())
            .bind(profile.first_name())
            .bind(profile.last_name())
            .bind(profile.avatar_url())
            .bind(profile.bio())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
    
    async fn get_by_user_id(
        &self,
        ctx: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<Option<Profile>, anyhow::Error> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| anyhow::anyhow!("Invalid TxContext context".to_string()))?;

        let row: Option<MySqlProfileRow> = sqlx::query_as(
            r#"
            SELECT user_id, first_name, last_name, avatar_url, bio
            FROM profiles
            WHERE user_id = ?
            "#
        )
            .bind(user_id.value())
            .fetch_optional(&mut *ctx.tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        match row {
            Some(row) => Ok(Some(Profile::try_from(row)?)),
            None => Ok(None),
        }
    }
    
    async fn exists(
        &self,
        ctx: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<bool, anyhow::Error> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| anyhow::anyhow!("Invalid TxContext context".to_string()))?;

        let row = sqlx::query(
            r#"
            SELECT 1
            FROM profiles
            WHERE user_id = ?
            LIMIT 1
            "#
        )
            .bind(user_id.value())
            .fetch_optional(&mut *ctx.tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(row.is_some())
    }
    
    async fn update(
        &self,
        ctx: &mut dyn TxContext,
        profile: &Profile
    ) -> Result<(), anyhow::Error> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| anyhow::anyhow!("Invalid TxContext context".to_string()))?;

        let row = MySqlProfileRow::from(profile);

        sqlx::query(
            r#"
            UPDATE profiles
            SET first_name = ?, last_name = ?, avatar_url = ?, bio = ?
            WHERE user_id = ?
            "#
        )
            .bind(row.first_name())
            .bind(row.last_name())
            .bind(row.avatar_url())
            .bind(row.bio())
            .bind(row.user_id())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
    
    async fn remove(
        &self,
        ctx: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<(), anyhow::Error> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| anyhow::anyhow!("Invalid TxContext context".to_string()))?;

        sqlx::query(
            r#"
            DELETE FROM profiles
            WHERE user_id = ?
            "#
        )
            .bind(user_id.value())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
}
