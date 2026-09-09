use async_trait::{ async_trait };

use domain::aggregates::otp::{
    Otp,
    value_objects::{ OtpProviderType, OtpProviderKey }
};

use application::contracts::cqrs::command::{ CommandContext };
use application::features::auth::commands::send_otp::{ SendOtpCommandService };

use crate::persistence::mysql::{
    contracts::cqrs::command::{ MySqlCommandContext },
    features::otps::rows::{
        MySqlOtpRow,
        value_objects::{ MySqlOtpProviderTypeRow, MySqlOtpProviderKeyRow }
    },
};

#[derive(Default)]
pub struct MySqlSendOtpCommandService;

#[async_trait]
impl SendOtpCommandService for MySqlSendOtpCommandService {
    async fn save_otp(&self, context: &mut dyn CommandContext, otp: &Otp) -> Result<(), anyhow::Error> {
        let tx = context.as_any_mut()
            .downcast_mut::<MySqlCommandContext>()
            .map(|context| context.tx_mut())
            .ok_or_else(|| anyhow::anyhow!("Invalid CommandContext".to_string()))?;

        let row: MySqlOtpRow = otp.into();

        sqlx::query(
            r#"
            INSERT INTO otps (id, code, provider_type, provider_key, expires_at)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
            .bind(row.id)
            .bind(row.code)
            .bind(row.provider_type)
            .bind(row.provider_key)
            .bind(row.expires_at)
            .execute(&mut **tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }

    async fn delete_old_otps(&self, ctx: &mut dyn CommandContext, provider_type: &OtpProviderType, provider_key: &OtpProviderKey) -> Result<(), anyhow::Error> {
        let tx = ctx.as_any_mut()
            .downcast_mut::<MySqlCommandContext>()
            .map(|context| context.tx_mut())
            .ok_or_else(|| anyhow::anyhow!("Invalid CommandContext".to_string()))?;

        let provider_type_row: MySqlOtpProviderTypeRow = provider_type.clone().into();
        let provider_key_row: MySqlOtpProviderKeyRow = provider_key.clone().into();
        
        sqlx::query(
            r#"
            DELETE FROM otps
            WHERE provider_type = ? AND provider_key = ?
            "#
        )
            .bind(provider_type_row)
            .bind(provider_key_row)
            .execute(&mut **tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
}
