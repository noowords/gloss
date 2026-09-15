use domain::aggregates::salons::salon_schedule_exception_interval::SalonScheduleExceptionInterval;

use super::value_objects::{
    MySqlSalonScheduleExceptionIntervalEndsAtRow,
    MySqlSalonScheduleExceptionIntervalExceptionIdRow,
    MySqlSalonScheduleExceptionIntervalIdRow,
    MySqlSalonScheduleExceptionIntervalStartsAtRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlSalonScheduleExceptionIntervalRow {
    pub id: MySqlSalonScheduleExceptionIntervalIdRow,
    pub exception_id: MySqlSalonScheduleExceptionIntervalExceptionIdRow,
    pub starts_at: MySqlSalonScheduleExceptionIntervalStartsAtRow,
    pub ends_at: MySqlSalonScheduleExceptionIntervalEndsAtRow
}

impl TryFrom<MySqlSalonScheduleExceptionIntervalRow> for SalonScheduleExceptionInterval {
    type Error = anyhow::Error;

    fn try_from(row: MySqlSalonScheduleExceptionIntervalRow) -> Result<Self, Self::Error> {
        SalonScheduleExceptionInterval::restore(
            uuid::Uuid::from(row.id).into(),
            uuid::Uuid::from(row.exception_id).into(),
            chrono::NaiveTime::from(row.starts_at).into(),
            chrono::NaiveTime::from(row.ends_at).into()
        )
    }
}

impl From<&SalonScheduleExceptionInterval> for MySqlSalonScheduleExceptionIntervalRow {
    fn from(entity: &SalonScheduleExceptionInterval) -> Self {
        Self {
            id: uuid::Uuid::from(entity.id()).into(),
            exception_id: uuid::Uuid::from(entity.exception_id()).into(),
            starts_at: chrono::NaiveTime::from(entity.starts_at()).into(),
            ends_at: chrono::NaiveTime::from(entity.ends_at()).into()
        }
    }
}
