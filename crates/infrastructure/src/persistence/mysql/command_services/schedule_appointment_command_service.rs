use async_trait::{ async_trait };

use domain::appointment::{ Appointment };

use application::contexts::{ TxContext };
use application::persistence::command_services::{ ScheduleAppointmentCommandService };

use crate::contexts::mysql::{ MySqlTxContext };
use crate::models::appointment::{ MySqlAppointmentModel };

#[derive(Default)]
pub struct MySqlScheduleAppointmentCommandService;

#[async_trait]
impl ScheduleAppointmentCommandService for MySqlScheduleAppointmentCommandService {
    async fn save_appointment(&self, ctx: &mut dyn TxContext, appointment: &Appointment) -> Result<(), anyhow::Error> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| anyhow::anyhow!("Invalid TxContext context".to_string()))?;

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
            .execute(&mut *ctx.tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
}
