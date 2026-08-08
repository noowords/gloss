use domain::aggregates::appointment::value_objects::{ AppointmentStatus };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlAppointmentStatusRow(String);

impl From<MySqlAppointmentStatusRow> for String {
    fn from(row: MySqlAppointmentStatusRow) -> Self {
        row.0
    }
}

impl From<String> for MySqlAppointmentStatusRow {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl TryFrom<MySqlAppointmentStatusRow> for AppointmentStatus {
    type Error = anyhow::Error;

    fn try_from(row: MySqlAppointmentStatusRow) -> Result<Self, Self::Error> {
        row.0.try_into()
    }
}

impl From<AppointmentStatus> for MySqlAppointmentStatusRow {
    fn from(entity: AppointmentStatus) -> Self {
        Self(entity.into())
    }
}
