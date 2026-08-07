use chrono::{ NaiveDate };

use domain::aggregates::appointment::value_objects::{ AppointmentDate };

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlAppointmentDateRow(NaiveDate);

impl From<MySqlAppointmentDateRow> for AppointmentDate {
    fn from(model: MySqlAppointmentDateRow) -> Self {
        model.0.into()
    }
}

impl From<AppointmentDate> for MySqlAppointmentDateRow {
    fn from(entity: AppointmentDate) -> Self {
        Self(entity.into())
    }
}
