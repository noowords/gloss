use async_trait::{ async_trait };

use domain::aggregates::user::value_objects::{ UserId };
use application::{
    contracts::cqrs::query::{ QueryContext },
    features::users::queries::get_by_id::{
        GetUserByIdQueryService,
        dtos::{ User }
    }
};

use crate::persistence::mysql::{
    contracts::cqrs::query::{ MySqlQueryContext },
    features::users::rows::value_objects::{ MySqlUserIdRow }
};

use super::dtos::{ MySqlUserRow };

#[derive(Default)]
pub struct MySqlGetUserByIdQueryService;

#[async_trait]
impl GetUserByIdQueryService for MySqlGetUserByIdQueryService {
    async fn get_user_by_id(&self, context: &dyn QueryContext, id: UserId) -> Result<Option<User>, anyhow::Error> {
        let pool = context.as_any()
            .downcast_ref::<MySqlQueryContext>()
            .map(|context| context.pool())
            .ok_or_else(|| anyhow::anyhow!("Invalid QueryContext"))?;

        let id_row: MySqlUserIdRow = id.into();

        let row: Option<MySqlUserRow> = sqlx::query_as(
            r#"
            SELECT
                u.id         AS id,
                u.role       AS role,
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
            .bind(id_row)
            .fetch_optional(pool)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(row.map(|row| row.into()))
    }
}
