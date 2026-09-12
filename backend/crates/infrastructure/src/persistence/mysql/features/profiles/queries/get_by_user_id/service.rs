use async_trait::{ async_trait };

use domain::aggregates::user::value_objects::{ UserId };
use application::{
    contracts::cqrs::query::{ QueryContext },
    features::account::queries::get_profile::{
        GetAccountProfileQueryService,
        dtos::{ Profile }
    }
};

use crate::persistence::mysql::{
    contracts::cqrs::query::{ MySqlQueryContext },
    features::users::rows::value_objects::{ MySqlUserIdRow }
};

use super::dtos::{ MySqlProfileRow };

#[derive(Default)]
pub struct MySqlGetProfileByUserIdQueryService;

#[async_trait]
impl GetAccountProfileQueryService for MySqlGetProfileByUserIdQueryService {
    async fn get_profile_by_user_id(&self, context: &dyn QueryContext, user_id: UserId) -> Result<Option<Profile>, anyhow::Error> {
        let pool = context.as_any()
            .downcast_ref::<MySqlQueryContext>()
            .map(|context| context.pool())
            .ok_or_else(|| anyhow::anyhow!("Invalid QueryContext"))?;

        let user_id_row: MySqlUserIdRow = user_id.into();

        let row: Option<MySqlProfileRow> = sqlx::query_as(
            r#"
            SELECT
                p.user_id     AS user_id,
                p.first_name  AS first_name,
                p.last_name   AS last_name,
                p.avatar_url  AS avatar_url,
                p.bio         AS bio
            FROM profiles p
            WHERE p.user_id = ?
            LIMIT 1
            "#
        )
            .bind(user_id_row)
            .fetch_optional(pool)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(row.map(|row| row.into()))
    }
}
