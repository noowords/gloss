use domain::aggregates::appointment::value_objects::{ AppointmentStatus };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlAppointmentStatusRow(String);

impl TryFrom<MySqlAppointmentStatusRow> for AppointmentStatus {
    type Error = anyhow::Error;

    fn try_from(model: MySqlAppointmentStatusRow) -> Result<Self, Self::Error> {
        model.0.try_into()
    }
}

impl From<AppointmentStatus> for MySqlAppointmentStatusRow {
    fn from(entity: AppointmentStatus) -> Self {
        Self(entity.into())
    }
}
