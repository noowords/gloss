use async_trait::{ async_trait };

use domain::user::{
    User,
    profile::{ Profile }
};

use application::contexts::{ TxContext };
use application::features::commands::register_user::{ RegisterUserCommandService };

use crate::persistence::mysql::{
    contexts::{ MySqlTxContext },
    models::{
        user::{ MySqlUserModel },
        profile::{ MySqlProfileModel }
    }
};

#[derive(Default)]
pub struct MySqlRegisterUserCommandService;

#[async_trait]
impl RegisterUserCommandService for MySqlRegisterUserCommandService {
    async fn save_user(&self, ctx: &mut dyn TxContext, user: &User) -> Result<(), anyhow::Error> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| anyhow::anyhow!("Invalid TxContext context".to_string()))?;

        let model: MySqlUserModel = user.into();

        sqlx::query(
            r#"
            INSERT INTO users (id, role, phone)
            VALUES (?, ?, ?)
            "#
        )
            .bind(model.id())
            .bind(model.role())
            .bind(model.phone())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }

    async fn save_profile(&self, ctx: &mut dyn TxContext, profile: &Profile) -> Result<(), anyhow::Error> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| anyhow::anyhow!("Invalid TxContext context".to_string()))?;

        let model: MySqlProfileModel = profile.into();

        sqlx::query(
            r#"
            INSERT INTO profiles (user_id, first_name, last_name, avatar_url, bio)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
            .bind(model.user_id())
            .bind(model.first_name())
            .bind(model.last_name())
            .bind(model.avatar_url())
            .bind(model.bio())
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
}
