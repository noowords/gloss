use domain::aggregates::appointment::value_objects::{ AppointmentDuration };

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlAppointmentDurationRow(u32);

impl From<MySqlAppointmentDurationRow> for u32 {
    fn from(row: MySqlAppointmentDurationRow) -> Self {
        row.0
    }
}

impl From<u32> for MySqlAppointmentDurationRow {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<MySqlAppointmentDurationRow> for AppointmentDuration {
    fn from(row: MySqlAppointmentDurationRow) -> Self {
        row.0.into()
    }
}

impl From<AppointmentDuration> for MySqlAppointmentDurationRow {
    fn from(entity: AppointmentDuration) -> Self {
        Self(entity.into())
    }
}
