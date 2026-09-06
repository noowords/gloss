use async_trait::{ async_trait };

use domain::aggregates::appointment::value_objects::{ AppointmentId };
use application::{
    contracts::cqrs::query::{ QueryContext },
    features::appointments::queries::get_by_id::{
        GetAppointmentByIdQueryService,
        dtos::{ Appointment }
    }
};

use crate::persistence::mysql::{
    contracts::cqrs::query::{ MySqlQueryContext },
    features::appointments::rows::value_objects::{ MySqlAppointmentIdRow }
};

use super::dtos::{ MySqlAppointmentRow };

#[derive(Default)]
pub struct MySqlGetAppointmentByIdQueryService;

#[async_trait]
impl GetAppointmentByIdQueryService for MySqlGetAppointmentByIdQueryService {
    async fn get_appointment_by_id(&self, context: &dyn QueryContext, id: AppointmentId) -> Result<Option<Appointment>, anyhow::Error> {
        let pool = context.as_any()
            .downcast_ref::<MySqlQueryContext>()
            .map(|context| context.pool())
            .ok_or_else(|| anyhow::anyhow!("Invalid QueryContext"))?;

        let id_row: MySqlAppointmentIdRow = id.into();

        let row: Option<MySqlAppointmentRow> = sqlx::query_as(
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
            WHERE u.id = ?
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
