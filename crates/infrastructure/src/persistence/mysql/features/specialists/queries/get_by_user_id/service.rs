use async_trait::{ async_trait };

use domain::aggregates::user::value_objects::{ UserId };
use application::{
    contracts::cqrs::query::{ QueryContext },
    features::specialists::queries::get_by_user_id::{
        GetSpecialistByUserIdQueryService,
        dtos::{ Specialist }
    }
};

use crate::persistence::mysql::{
    contracts::cqrs::query::{ MySqlQueryContext },
    features::users::rows::value_objects::{ MySqlUserIdRow }
};

use super::dtos::{ MySqlSpecialistRow };

#[derive(Default)]
pub struct MySqlGetSpecialistByUserIdQueryService;

#[async_trait]
impl GetSpecialistByUserIdQueryService for MySqlGetSpecialistByUserIdQueryService {
    async fn get_specialist_by_user_id(&self, context: &dyn QueryContext, user_id: UserId) -> Result<Option<Specialist>, anyhow::Error> {
        let pool = context.as_any()
            .downcast_ref::<MySqlQueryContext>()
            .map(|context| context.pool())
            .ok_or_else(|| anyhow::anyhow!("Invalid QueryContext"))?;

        let user_id_row: MySqlUserIdRow = user_id.into();

        let row: Option<MySqlSpecialistRow> = sqlx::query_as(
            r#"
            SELECT
                u.id         AS id,
                p.first_name AS first_name,
                p.last_name  AS last_name,
                p.avatar_url AS avatar_url,
                p.bio        AS bio
            FROM users u
            LEFT JOIN profiles p ON u.id = p.user_id
            WHERE u.id = ? AND u.deleted_at IS NULL
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
