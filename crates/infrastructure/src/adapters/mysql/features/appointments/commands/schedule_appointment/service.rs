use async_trait::{ async_trait };

use domain::aggregates::appointment::{ Appointment };

use application::contracts::command::{ CommandContext };
use application::features::appointments::commands::schedule_appointment::{ ScheduleAppointmentCommandService };

use crate::adapters::mysql::interfaces::command::{ MySqlCommandContext };
use crate::models::mysql::appointment::{ MySqlAppointmentModel };

#[derive(Default)]
pub struct MySqlScheduleAppointmentCommandService;

#[async_trait]
impl ScheduleAppointmentCommandService for MySqlScheduleAppointmentCommandService {
    async fn save_appointment(&self, context: &mut dyn CommandContext, appointment: &Appointment) -> Result<(), anyhow::Error> {
        let tx = context.as_any_mut()
            .downcast_mut::<MySqlCommandContext>()
            .map(|context| context.tx_mut())
            .ok_or_else(|| anyhow::anyhow!("Invalid CommandContext".to_string()))?;

        let model: MySqlAppointmentModel = appointment.into();

        sqlx::query(
            r#"
            INSERT INTO appointments (id, master_id, client_id, date, time, status, created_at)
            VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
            "#
        )
            .bind(model.id())
            .bind(model.master_id())
            .bind(model.client_id())
            .bind(model.date())
            .bind(model.time().format("%H:%M:%S").to_string()) // ?
            .bind(model.status())
            .execute(&mut **tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
}
