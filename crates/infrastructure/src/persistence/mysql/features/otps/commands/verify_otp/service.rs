use async_trait::{ async_trait };

use domain::aggregates::{
    user::{
        User,
        value_objects::{ UserId }
    },
    user_identity::{ UserIdentity },
    otp::{
        Otp,
        value_objects::{ OtpProviderType, OtpProviderKey, OtpCode }
    }
};

use application::contracts::cqrs::command::{ CommandContext };
use application::features::auth::commands::verify_otp::{ VerifyOtpCommandService };

use crate::persistence::mysql::{
    contracts::cqrs::command::{ MySqlCommandContext },
    features::{
        users::rows::{
            MySqlUserRow,
            value_objects::{ MySqlUserIdRow }
        },
        user_identities::rows::{ MySqlUserIdentityRow },
        otps::rows::{
            MySqlOtpRow,
            value_objects::{ MySqlOtpProviderTypeRow, MySqlOtpProviderKeyRow, MySqlOtpCodeRow }
        }
    },
};

#[derive(Default)]
pub struct MySqlVerifyOtpCommandService;

#[async_trait]
impl VerifyOtpCommandService for MySqlVerifyOtpCommandService {
    async fn get_otp(&self, context: &mut dyn CommandContext, provider_type: &OtpProviderType, provider_key: &OtpProviderKey, code: &OtpCode) -> Result<Option<Otp>, anyhow::Error> {
        let tx = context.as_any_mut()
            .downcast_mut::<MySqlCommandContext>()
            .map(|context| context.tx_mut())
            .ok_or_else(|| anyhow::anyhow!("Invalid CommandContext".to_string()))?;

        let provider_type: MySqlOtpProviderTypeRow = provider_type.clone().into();
        let provider_key: MySqlOtpProviderKeyRow = provider_key.clone().into();
        let code: MySqlOtpCodeRow = code.clone().into();
        
        let row: Option<MySqlOtpRow> = sqlx::query_as(
            r#"
            SELECT
                id,
                provider_type,
                provider_key,
                code,
                expires_at
            FROM otps
            WHERE provider_type = ? AND provider_key = ? AND code = ?
            "#
        )
            .bind(provider_type)
            .bind(provider_key)
            .bind(code)
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        if let Some(r) = row {
            let p_type = r.provider_type.try_into().map_err(|_| anyhow::anyhow!("Invalid type"))?;
            return Ok(Some(Otp::restore(r.id.into(), p_type, r.provider_key.into(), r.code.into(), r.expires_at.into())));
        }
        
        Ok(None)
    }

    async fn remove_otps(&self, ctx: &mut dyn CommandContext, provider_type: &OtpProviderType, provider_key: &OtpProviderKey) -> Result<(), anyhow::Error> {
        let tx = ctx.as_any_mut()
            .downcast_mut::<MySqlCommandContext>()
            .map(|context| context.tx_mut())
            .ok_or_else(|| anyhow::anyhow!("Invalid CommandContext".to_string()))?;

        let provider_type: MySqlOtpProviderTypeRow = provider_type.clone().into();
        let provider_key: MySqlOtpProviderKeyRow = provider_key.clone().into();

        sqlx::query(
            r#"
            DELETE FROM otps
            WHERE provider_type = ? AND provider_key = ?
            "#
        )
            .bind(provider_type)
            .bind(provider_key)
            .execute(&mut **tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }

    async fn get_user_id(&self, ctx: &mut dyn CommandContext, provider_type: &OtpProviderType, provider_key: &OtpProviderKey) -> Result<Option<UserId>, anyhow::Error> {
        let tx = ctx.as_any_mut()
            .downcast_mut::<MySqlCommandContext>()
            .map(|context| context.tx_mut())
            .ok_or_else(|| anyhow::anyhow!("Invalid CommandContext".to_string()))?;

        let provider_type: MySqlOtpProviderTypeRow = provider_type.clone().into();
        let provider_key: MySqlOtpProviderKeyRow = provider_key.clone().into();

        let row: Option<MySqlUserIdRow> = sqlx::query_as(
            r#"
            SELECT user_id
            FROM user_identities
            WHERE provider_type = ? AND provider_key = ?
            LIMIT 1
            "#
        )
            .bind(provider_type)
            .bind(provider_key)
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(row.map(|row| row.into()))
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

    async fn save_user(&self, ctx: &mut dyn CommandContext, user: &User) -> Result<(), anyhow::Error> {
        let tx = ctx.as_any_mut()
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

    async fn save_user_identity(&self, ctx: &mut dyn CommandContext, user_identity: &UserIdentity) -> Result<(), anyhow::Error> {
        let tx = ctx.as_any_mut()
            .downcast_mut::<MySqlCommandContext>()
            .map(|context| context.tx_mut())
            .ok_or_else(|| anyhow::anyhow!("Invalid CommandContext".to_string()))?;

        let row: MySqlUserIdentityRow = user_identity.try_into()?;
        
        sqlx::query(
            r#"
            INSERT INTO user_identities (id, user_id, provider_type, provider_key, provider_data)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(row.id)
        .bind(row.user_id)
        .bind(row.provider_type)
        .bind(row.provider_key)
        .bind(row.provider_data)
        .execute(&mut **tx)
        .await
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
}
