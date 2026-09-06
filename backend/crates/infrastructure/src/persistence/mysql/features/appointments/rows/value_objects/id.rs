use uuid::{ Uuid };

use domain::aggregates::appointment::value_objects::{ AppointmentId };

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlAppointmentIdRow(Uuid);

impl From<MySqlAppointmentIdRow> for Uuid {
    fn from(row: MySqlAppointmentIdRow) -> Self {
        row.0
    }
}

impl From<Uuid> for MySqlAppointmentIdRow {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<MySqlAppointmentIdRow> for AppointmentId {
    fn from(row: MySqlAppointmentIdRow) -> Self {
        row.0.into()
    }
}

impl From<AppointmentId> for MySqlAppointmentIdRow {
    fn from(entity: AppointmentId) -> Self {
        Self(entity.into())
    }
}
