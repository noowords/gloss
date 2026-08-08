use async_trait::{ async_trait };

use domain::aggregates::user::value_objects::{ UserId };
use application::{
    contracts::cqrs::query::{ QueryContext },
    features::specialists::queries::get_services_by_user_id::{
        GetSpecialistServicesByUserIdQueryService,
        dtos::{ Service }
    }
};

use crate::persistence::mysql::{
    contracts::cqrs::query::{ MySqlQueryContext },
    features::users::rows::value_objects::{ MySqlUserIdRow }
};

use super::dtos::{ MySqlServiceRow };

#[derive(Default)]
pub struct MySqlGetSpecialistServicesByUserIdQueryService;

#[async_trait]
impl GetSpecialistServicesByUserIdQueryService for MySqlGetSpecialistServicesByUserIdQueryService {
    async fn get_specialist_services_by_user_id(&self, context: &dyn QueryContext, user_id: UserId) -> Result<Vec<Service>, anyhow::Error> {
        let pool = context.as_any()
            .downcast_ref::<MySqlQueryContext>()
            .map(|context| context.pool())
            .ok_or_else(|| anyhow::anyhow!("Invalid QueryContext"))?;

        let user_id_row: MySqlUserIdRow = user_id.into();

        let rows: Vec<MySqlServiceRow> = sqlx::query_as(
            r#"
            SELECT
                id,
                specialist_id,
                name,
                price,
                duration,
                is_active
            FROM services
            WHERE specialist_id = ?
              AND is_active = 1
            "#
        )
            .bind(user_id_row)
            .fetch_all(pool)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }
}
