use domain::aggregates::appointments::appointment::{ Appointment };
use domain::aggregates::appointments::appointment_service::AppointmentService;

use super::value_objects::{
    MySqlAppointmentCancellationReasonRow,
    MySqlAppointmentCancelledAtRow,
    MySqlAppointmentClientIdRow,
    MySqlAppointmentEndsAtRow,
    MySqlAppointmentIdRow,
    MySqlAppointmentSalonIdRow,
    MySqlAppointmentSpecialistIdRow,
    MySqlAppointmentStartsAtRow,
    MySqlAppointmentStatusRow,
    MySqlAppointmentTotalDurationMinutesSnapshotRow,
    MySqlAppointmentTotalPriceSnapshotRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlAppointmentRow {
    pub id: MySqlAppointmentIdRow,
    pub client_id: MySqlAppointmentClientIdRow,
    pub salon_id: MySqlAppointmentSalonIdRow,
    pub specialist_id: MySqlAppointmentSpecialistIdRow,
    pub starts_at: MySqlAppointmentStartsAtRow,
    pub ends_at: MySqlAppointmentEndsAtRow,
    pub status: MySqlAppointmentStatusRow,
    pub total_price_snapshot: MySqlAppointmentTotalPriceSnapshotRow,
    pub total_duration_minutes_snapshot: MySqlAppointmentTotalDurationMinutesSnapshotRow,
    pub cancelled_at: Option<MySqlAppointmentCancelledAtRow>,
    pub cancellation_reason: Option<MySqlAppointmentCancellationReasonRow>,
    #[sqlx(skip)]
    pub services: Vec<AppointmentService>
}

impl TryFrom<MySqlAppointmentRow> for Appointment {
    type Error = anyhow::Error;

    fn try_from(row: MySqlAppointmentRow) -> Result<Self, Self::Error> {
        Self::restore(
            row.id.into(),
            uuid::Uuid::from(row.client_id).into(),
            uuid::Uuid::from(row.salon_id).into(),
            uuid::Uuid::from(row.specialist_id).into(),
            chrono::NaiveDateTime::from(row.starts_at).into(),
            chrono::NaiveDateTime::from(row.ends_at).into(),
            row.status.try_into()?,
            bigdecimal::BigDecimal::from(row.total_price_snapshot).try_into()?,
            u16::from(row.total_duration_minutes_snapshot).try_into()?,
            row.cancelled_at.map(|value| chrono::NaiveDateTime::from(value).into()),
            row.cancellation_reason.map(|value| String::from(value).try_into()).transpose()?,
            row.services
        )
    }
}

impl From<&Appointment> for MySqlAppointmentRow {
    fn from(entity: &Appointment) -> Self {
        Self {
            id: entity.id().into(),
            client_id: uuid::Uuid::from(entity.client_id()).into(),
            salon_id: uuid::Uuid::from(entity.salon_id()).into(),
            specialist_id: uuid::Uuid::from(entity.specialist_id()).into(),
            starts_at: chrono::NaiveDateTime::from(entity.starts_at()).into(),
            ends_at: chrono::NaiveDateTime::from(entity.ends_at()).into(),
            status: entity.status().into(),
            total_price_snapshot: bigdecimal::BigDecimal::from(entity.total_price_snapshot()).into(),
            total_duration_minutes_snapshot: u16::from(entity.total_duration_minutes_snapshot()).into(),
            cancelled_at: entity.cancelled_at().map(|value| chrono::NaiveDateTime::from(value).into()),
            cancellation_reason: entity.cancellation_reason().map(|value| String::from(value).into()),
            services: entity.services().to_vec()
        }
    }
}
