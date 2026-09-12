use async_trait::{ async_trait };

use domain::aggregates::appointment::{ Appointment };

use application::{
    contracts::cqrs::command::{ CommandContext },
    features::appointments::commands::schedule::{ ScheduleAppointmentCommandService },
};

use crate::persistence::mysql::{
    contracts::cqrs::command::{ MySqlCommandContext },
    features::appointments::rows::{ MySqlAppointmentRow },
};

#[derive(Default)]
pub struct MySqlAppointmentCommandService;

#[async_trait]
impl ScheduleAppointmentCommandService for MySqlAppointmentCommandService {
    async fn save_appointment(&self, context: &mut dyn CommandContext, appointment: &Appointment) -> Result<(), anyhow::Error> {
        let tx = context.as_any_mut()
            .downcast_mut::<MySqlCommandContext>()
            .map(|context| context.tx_mut())
            .ok_or_else(|| anyhow::anyhow!("Invalid CommandContext".to_string()))?;

        let row: MySqlAppointmentRow = appointment.into();

        sqlx::query(
            r#"
            INSERT INTO appointments (id, specialist_id, client_id, date, time, duration, status, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
            "#
        )
            .bind(row.id)
            .bind(row.specialist_id)
            .bind(row.client_id)
            .bind(row.date)
            .bind(row.time)
            .bind(row.duration)
            .bind(row.status)
            .execute(&mut **tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
}
