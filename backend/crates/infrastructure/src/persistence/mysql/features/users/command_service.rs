use async_trait::{ async_trait };

use domain::aggregates::user::{
    User,
    value_objects::{ UserId }
};

use application::{
    contracts::cqrs::command::{ CommandContext },
    features::auth::commands::{
        refresh_tokens::{ RefreshTokensCommandService }
    }
};

use crate::persistence::mysql::{
    contracts::cqrs::command::{ MySqlCommandContext },
    features::users::rows::{
        MySqlUserRow,
        value_objects::{ MySqlUserIdRow }
    }
};

#[derive(Default)]
pub struct MySqlUserCommandService;

#[async_trait]
impl RefreshTokensCommandService for MySqlUserCommandService {
    async fn get_user_by_id(&self, ctx: &mut dyn CommandContext, user_id: &UserId) -> Result<Option<User>, anyhow::Error> {
        let tx = ctx.as_any_mut()
            .downcast_mut::<MySqlCommandContext>()
            .map(|context| context.tx_mut())
            .ok_or_else(|| anyhow::anyhow!("Invalid CommandContext".to_string()))?;

        let user_id_row: MySqlUserIdRow = user_id.clone().into();

        let row: Option<MySqlUserRow> = sqlx::query_as(
            r#"
            SELECT
                u.id         AS id,
                u.role       AS role
            FROM users u
            WHERE u.id = ?
            LIMIT 1
            "#
        )
            .bind(user_id_row)
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let user = match row {
            Some(r) => {
                let role = r.role.try_into()
                    .map_err(|_| anyhow::anyhow!("Invalid role".to_string()))?;
                Some(User::restore(r.id.into(), role))
            }
            None => None
        };

        Ok(user)
    }

    async fn check_profile_exists(&self, ctx: &mut dyn CommandContext, user_id: &UserId) -> Result<bool, anyhow::Error> {
        let tx = ctx.as_any_mut()
            .downcast_mut::<MySqlCommandContext>()
            .map(|context| context.tx_mut())
            .ok_or_else(|| anyhow::anyhow!("Invalid CommandContext".to_string()))?;

        let user_id: MySqlUserIdRow = user_id.clone().into();

        let row: Option<MySqlUserIdRow> = sqlx::query_as(
            r#"
            SELECT user_id
            FROM profiles
            WHERE user_id = ?
            LIMIT 1
            "#
        )
            .bind(user_id)
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(row.is_some())
    }
}
