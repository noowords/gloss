use async_trait::{ async_trait };

use application::{
    contracts::cqrs::query::{ QueryContext },
    features::users::queries::get::{
        GetUsersQueryService,
        dtos::{ User }
    }
};

use crate::persistence::mysql::contracts::cqrs::query::{ MySqlQueryContext };

use super::dtos::{ MySqlUserRow };

#[derive(Default)]
pub struct MySqlGetUsersQueryService;

#[async_trait]
impl GetUsersQueryService for MySqlGetUsersQueryService {
    async fn get_users(&self, context: &dyn QueryContext) -> Result<Vec<User>, anyhow::Error> {
        let pool = context.as_any()
            .downcast_ref::<MySqlQueryContext>()
            .map(|context| context.pool())
            .ok_or_else(|| anyhow::anyhow!("Invalid QueryContext"))?;

        let rows: Vec<MySqlUserRow> = sqlx::query_as(
            r#"
            SELECT
                u.id         AS id,
                u.role       AS role,
                p.first_name AS first_name,
                p.last_name  AS last_name,
                p.avatar_url AS avatar_url,
                p.bio        AS bio
            FROM users u
            INNER JOIN profiles p ON u.id = p.user_id
            WHERE u.deleted_at IS NULL
            ORDER BY u.created_at DESC
            "#
        )
            .fetch_all(pool)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }
}
