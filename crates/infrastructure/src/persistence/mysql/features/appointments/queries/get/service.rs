use async_trait::{ async_trait };

use application::{
    contracts::cqrs::query::{ QueryContext },
    features::appointments::queries::get::{
        GetAppointmentsQueryService,
        dtos::{ Appointment }
    }
};

use crate::persistence::mysql::contracts::cqrs::query::{ MySqlQueryContext };

use super::dtos::{ MySqlAppointmentRow };

#[derive(Default)]
pub struct MySqlGetAppointmentsQueryService;

#[async_trait]
impl GetAppointmentsQueryService for MySqlGetAppointmentsQueryService {
    async fn get_appointments(&self, context: &dyn QueryContext) -> Result<Vec<Appointment>, anyhow::Error> {
        let pool = context.as_any()
            .downcast_ref::<MySqlQueryContext>()
            .map(|context| context.pool())
            .ok_or_else(|| anyhow::anyhow!("Invalid QueryContext"))?;

        let rows: Vec<MySqlAppointmentRow> = sqlx::query_as(
            r#"
            SELECT
                a.id               AS id,
                a.specialist_id    AS specialist_id,
                a.client_id        AS client_id,
                a.date             AS date,
                a.time             AS time,
                a.duration         AS duration,
                a.status           AS status
            FROM appointments a
            ORDER BY a.created_at DESC
            "#
        )
            .fetch_all(pool)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }
}
