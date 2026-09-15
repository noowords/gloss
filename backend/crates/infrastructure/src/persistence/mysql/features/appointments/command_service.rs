use async_trait::{ async_trait };

use domain::aggregates::appointments::appointment::{ Appointment };

use application::{
    contracts::cqrs::command::{ CommandContext },
    features::appointments::commands::schedule::{ ScheduleAppointmentCommandService },
};

use crate::persistence::mysql::{
    contracts::cqrs::command::{ MySqlCommandContext },
    features::{
        appointments::rows::{ MySqlAppointmentRow },
        appointment_services::rows::{ MySqlAppointmentServiceRow }
    },
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
            INSERT INTO appointments (id, client_id, salon_id, specialist_id, starts_at, ends_at, status, total_price_snapshot, total_duration_minutes_snapshot, cancelled_at, cancellation_reason)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
            .bind(row.id)
            .bind(row.client_id)
            .bind(row.salon_id)
            .bind(row.specialist_id)
            .bind(row.starts_at)
            .bind(row.ends_at)
            .bind(row.status)
            .bind(row.total_price_snapshot)
            .bind(row.total_duration_minutes_snapshot)
            .bind(row.cancelled_at)
            .bind(row.cancellation_reason)
            .execute(&mut **tx)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        for service in appointment.services() {
            let service_row: MySqlAppointmentServiceRow = service.into();

            sqlx::query(
                r#"
                INSERT INTO appointment_services (
                    appointment_id,
                    service_id,
                    role,
                    service_name_snapshot,
                    price_snapshot,
                    duration_minutes_snapshot
                )
                VALUES (?, ?, ?, ?, ?, ?)
                "#
            )
                .bind(service_row.appointment_id)
                .bind(service_row.service_id)
                .bind(service_row.role)
                .bind(service_row.service_name_snapshot)
                .bind(service_row.price_snapshot)
                .bind(service_row.duration_minutes_snapshot)
                .execute(&mut **tx)
                .await
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        }

        Ok(())
    }
}
