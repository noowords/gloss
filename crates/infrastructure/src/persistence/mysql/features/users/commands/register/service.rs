use async_trait::{ async_trait };

use domain::aggregates::{
    user::{ User },
    profile::{ Profile }
};

use application::contracts::cqrs::command::{ CommandContext };
use application::features::users::commands::register::{ RegisterUserCommandService };

use crate::persistence::mysql::{
    contracts::cqrs::command::{ MySqlCommandContext },
    features::{
        users::rows::{ MySqlUserRow },
        profiles::rows::{ MySqlProfileRow },
    },
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

        let row: MySqlUserRow = user.into();

        sqlx::query(
            r#"
            INSERT INTO users (id, role)
            VALUES (?, ?)
            "#
        )
            .bind(row.id)
            .bind(row.role)
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

        let row: MySqlProfileRow = profile.into();

        sqlx::query(
            r#"
            INSERT INTO profiles (user_id, first_name, last_name, avatar_url, bio)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
            .bind(row.user_id)
            .bind(row.first_name)
            .bind(row.last_name)
            .bind(row.avatar_url)
            .bind(row.bio)
            .execute(&mut **tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
}
