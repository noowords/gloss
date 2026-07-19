use std::cell::{ UnsafeCell };
use async_trait::{ async_trait };
use sqlx::{ Transaction, MySql };

use domain::models::appointment::{
    Appointment, AppointmentRepository,
    value_objects::{ AppointmentId }
};

use super::super::models::{ MySqlAppointmentRow };

pub struct MySqlAppointmentRepository<'a> {
    tx: UnsafeCell<&'a mut Transaction<'static, MySql>>
}

unsafe impl<'a> Send for MySqlAppointmentRepository<'a> {}
unsafe impl<'a> Sync for MySqlAppointmentRepository<'a> {}

impl<'a> MySqlAppointmentRepository<'a> {
    pub fn new(tx: &'a mut Transaction<'static, MySql>) -> Self {
        Self { tx: UnsafeCell::new(tx) }
    }

    #[inline(always)]
    fn tx_mut(&self) -> &mut sqlx::MySqlConnection {
        unsafe {
            let tx_ref = &mut *self.tx.get();
            tx_ref.as_mut()
        }
    }
}

#[async_trait]
impl<'a> AppointmentRepository for MySqlAppointmentRepository<'a> {
    async fn create(&self, appointment: &Appointment) -> Result<(), anyhow::Error> {
        sqlx::query(
            r#"
            INSERT INTO appointments (id, master_id, client_id, date, time, status, created_at)
            VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
            "#
        )
            .bind(appointment.id().value())
            .bind(appointment.master_id().value())
            .bind(appointment.client_id().value())
            .bind(appointment.date())
            .bind(appointment.time().format("%H:%M:%S").to_string())
            .bind(appointment.status().as_str())
            .execute(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
    
    async fn get_by_id(&self, id: AppointmentId) -> Result<Option<Appointment>, anyhow::Error> {
        let row: Option<MySqlAppointmentRow> = sqlx::query_as(
            r#"
            SELECT id, master_id, client_id, date, time, status
            FROM appointments
            WHERE id = ?
            "#
        )
            .bind(id.value())
            .fetch_optional(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        match row {
            Some(row) => Ok(Some(Appointment::try_from(row)?)),
            None => Ok(None)
        }
    }
    
    async fn exists(&self, id: AppointmentId) -> Result<bool, anyhow::Error> {
        let row = sqlx::query(
            r#"
            SELECT 1
            FROM appointments
            WHERE id = ?
            LIMIT 1
            "#
        )
            .bind(id.value())
            .fetch_optional(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        
        Ok(row.is_some())
    }
    
    async fn update(&self, appointment: &Appointment) -> Result<(), anyhow::Error> {
        let row = MySqlAppointmentRow::from(appointment);

        sqlx::query(
            r#"
            UPDATE appointments
            SET status = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#
        )
            .bind(row.status())
            .bind(row.id())
            .execute(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        
        Ok(())
    }
    
    async fn remove(&self, id: AppointmentId) -> Result<(), anyhow::Error> {
        sqlx::query(
            r#"
            DELETE FROM appointments
            WHERE id = ?
            "#
        )
            .bind(id.value())
            .execute(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        
        Ok(())
    }
}
