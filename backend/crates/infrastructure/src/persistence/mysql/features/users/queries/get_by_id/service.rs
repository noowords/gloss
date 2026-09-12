use async_trait::{ async_trait };

use domain::aggregates::user::value_objects::{ UserId };
use application::{
    contracts::cqrs::query::{ QueryContext },
    features::account::queries::get::{
        GetAccountQueryService,
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
impl GetAccountQueryService for MySqlGetUserByIdQueryService {
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
                u.role       AS role
            FROM users u
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
