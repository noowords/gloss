use async_trait::{ async_trait };

use domain::aggregates::{
    user::value_objects::{ UserId },
    profile::{ Profile }
};

use application::{
    contracts::cqrs::command::{ CommandContext },
    features::account::commands::{
        create_profile::{ CreateAccountProfileCommandService },
        update_profile::{ UpdateAccountProfileCommandService }
    }
};

use crate::persistence::mysql::{
    contracts::cqrs::command::{ MySqlCommandContext },
    features::{
        users::rows::value_objects::{ MySqlUserIdRow },
        profiles::rows::{ MySqlProfileRow }
    }
};

#[derive(Default)]
pub struct MySqlProfileCommandService;

#[async_trait]
impl CreateAccountProfileCommandService for MySqlProfileCommandService {
    async fn create_profile(&self, context: &mut dyn CommandContext, profile: &Profile) -> Result<(), anyhow::Error> {
        let tx = context.as_any_mut()
            .downcast_mut::<MySqlCommandContext>()
            .map(|context| context.tx_mut())
            .ok_or_else(|| anyhow::anyhow!("Invalid CommandContext".to_string()))?;
        
        let profile_row: MySqlProfileRow = profile.into();

        sqlx::query(
            r#"
            INSERT INTO profiles (user_id, first_name, last_name, avatar_url, bio)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
            .bind(profile_row.user_id)
            .bind(profile_row.first_name)
            .bind(profile_row.last_name)
            .bind(profile_row.avatar_url)
            .bind(profile_row.bio)
            .execute(&mut **tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl UpdateAccountProfileCommandService for MySqlProfileCommandService {
    async fn update_profile(&self, context: &mut dyn CommandContext, user_id: &UserId, profile: &Profile) -> Result<(), anyhow::Error> {
        let tx = context.as_any_mut()
            .downcast_mut::<MySqlCommandContext>()
            .map(|context| context.tx_mut())
            .ok_or_else(|| anyhow::anyhow!("Invalid CommandContext".to_string()))?;

        let user_id_row: MySqlUserIdRow = user_id.clone().into();
        let profile_row: MySqlProfileRow = profile.into();

        sqlx::query(
            r#"
            UPDATE profiles
            SET first_name = ?, last_name = ?, avatar_url = ?, bio = ?, updated_at = NOW()
            WHERE user_id = ?
            "#
        )
            .bind(profile_row.first_name)
            .bind(profile_row.last_name)
            .bind(profile_row.avatar_url)
            .bind(profile_row.bio)
            .bind(user_id_row)
            .execute(&mut **tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
}
