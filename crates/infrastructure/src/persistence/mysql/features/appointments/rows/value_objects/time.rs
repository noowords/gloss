use chrono::{ NaiveTime };

use domain::aggregates::appointment::value_objects::{ AppointmentTime };

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlAppointmentTimeRow(NaiveTime);

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
