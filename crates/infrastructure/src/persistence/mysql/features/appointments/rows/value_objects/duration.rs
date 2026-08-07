use domain::aggregates::appointment::value_objects::{ AppointmentDuration };

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlAppointmentDurationRow(u32);

impl From<MySqlAppointmentDurationRow> for AppointmentDuration {
    fn from(model: MySqlAppointmentDurationRow) -> Self {
        model.0.into()
    }
}

impl From<AppointmentDuration> for MySqlAppointmentDurationRow {
    fn from(entity: AppointmentDuration) -> Self {
        Self(entity.into())
    }
}
