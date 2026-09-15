use chrono::NaiveTime;

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
