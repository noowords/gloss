use chrono::NaiveDate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlAppointmentDateRow(NaiveDate);

impl From<MySqlAppointmentDateRow> for NaiveDate {
    fn from(row: MySqlAppointmentDateRow) -> Self {
        row.0
    }
}

impl From<NaiveDate> for MySqlAppointmentDateRow {
    fn from(value: NaiveDate) -> Self {
        Self(value)
    }
}
