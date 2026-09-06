use async_trait::{ async_trait };

use domain::aggregates::user::value_objects::{ UserId };
use application::{
    contracts::cqrs::query::{ QueryContext },
    features::users::queries::get_profile_by_id::{
        GetUserProfileByIdQueryService,
        dtos::{ Profile }
    }
};

use crate::persistence::mysql::{
    contracts::cqrs::query::{ MySqlQueryContext },
    features::users::rows::value_objects::{ MySqlUserIdRow }
};

use super::dtos::{ MySqlProfileRow };

#[derive(Default)]
pub struct MySqlGetUserProfileByIdQueryService;

#[async_trait]
impl GetUserProfileByIdQueryService for MySqlGetUserProfileByIdQueryService {
    async fn get_user_profile_by_id(&self, context: &dyn QueryContext, id: UserId) -> Result<Option<Profile>, anyhow::Error> {
        let pool = context.as_any()
            .downcast_ref::<MySqlQueryContext>()
            .map(|context| context.pool())
            .ok_or_else(|| anyhow::anyhow!("Invalid QueryContext"))?;

        let id_row: MySqlUserIdRow = id.into();

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
            .bind(id_row)
            .fetch_optional(pool)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(row.map(|row| row.into()))
    }
}
