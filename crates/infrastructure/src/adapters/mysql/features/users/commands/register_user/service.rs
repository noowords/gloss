use async_trait::{ async_trait };

use domain::aggregates::user::{
    User,
    profile::{ Profile }
};

use application::interfaces::command::{ CommandContext };
use application::features::users::commands::register_user::{ RegisterUserCommandService };

use crate::adapters::mysql::interfaces::command::{ MySqlCommandContext };
use crate::models::mysql::{
    user::{ MySqlUserModel },
    profile::{ MySqlProfileModel }
};

#[derive(Default)]
pub struct MySqlRegisterUserCommandService;

#[async_trait]
impl RegisterUserCommandService for MySqlRegisterUserCommandService {
    async fn save_user(&self, context: &mut dyn CommandContext, user: &User) -> Result<(), anyhow::Error> {
        let tx = context.as_any_mut()
            .downcast_mut::<MySqlCommandContext>()
            .map(|context| context.tx_mut())
            .ok_or_else(|| anyhow::anyhow!("Invalid CommandContext".to_string()))?;

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
            .execute(&mut **tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }

    async fn save_profile(&self, context: &mut dyn CommandContext, profile: &Profile) -> Result<(), anyhow::Error> {
        let tx = context.as_any_mut()
            .downcast_mut::<MySqlCommandContext>()
            .map(|context| context.tx_mut())
            .ok_or_else(|| anyhow::anyhow!("Invalid CommandContext".to_string()))?;

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
            .execute(&mut **tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
}
