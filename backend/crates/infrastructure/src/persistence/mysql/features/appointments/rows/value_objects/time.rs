use chrono::{ NaiveTime };

use domain::aggregates::appointment::value_objects::{ AppointmentTime };

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlAppointmentTimeRow(NaiveTime);

impl From<MySqlAppointmentTimeRow> for NaiveTime {
    fn from(row: MySqlAppointmentTimeRow) -> Self {
        row.0
    }
}

impl From<NaiveTime> for MySqlAppointmentTimeRow {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}

impl From<MySqlAppointmentTimeRow> for AppointmentTime {
    fn from(row: MySqlAppointmentTimeRow) -> Self {
        row.0.into()
    }
}

impl From<AppointmentTime> for MySqlAppointmentTimeRow {
    fn from(entity: AppointmentTime) -> Self {
        Self(entity.into())
    }
}
