use uuid::{ Uuid };

use domain::aggregates::appointment::value_objects::{ AppointmentId };

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlAppointmentIdRow(Uuid);

impl From<MySqlAppointmentIdRow> for AppointmentId {
    fn from(model: MySqlAppointmentIdRow) -> Self {
        model.0.into()
    }
}

impl From<AppointmentId> for MySqlAppointmentIdRow {
    fn from(entity: AppointmentId) -> Self {
        Self(entity.into())
    }
}
