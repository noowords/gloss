use async_trait::{ async_trait };

use application::{
    contracts::cqrs::query::{ QueryContext },
    features::specialists::queries::get::{
        GetSpecialistsQueryService,
        dtos::{ Specialist }
    }
};

use crate::persistence::mysql::contracts::cqrs::query::{ MySqlQueryContext };

use super::dtos::{ MySqlSpecialistRow };

#[derive(Default)]
pub struct MySqlGetSpecialistsQueryService;

#[async_trait]
impl GetSpecialistsQueryService for MySqlGetSpecialistsQueryService {
    async fn get_specialists(&self, context: &dyn QueryContext) -> Result<Vec<Specialist>, anyhow::Error> {
        let pool = context.as_any()
            .downcast_ref::<MySqlQueryContext>()
            .map(|context| context.pool())
            .ok_or_else(|| anyhow::anyhow!("Invalid QueryContext"))?;

        let rows: Vec<MySqlSpecialistRow> = sqlx::query_as(
            r#"
            SELECT
                u.id         AS id,
                p.first_name AS first_name,
                p.last_name  AS last_name,
                p.avatar_url AS avatar_url,
                p.bio        AS bio
            FROM users u
            LEFT JOIN profiles p ON u.id = p.user_id
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
