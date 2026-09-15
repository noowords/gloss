use domain::aggregates::specialists::specialist_schedule_interval::SpecialistScheduleInterval;

use super::value_objects::{
    MySqlSpecialistScheduleIntervalEndsAtRow,
    MySqlSpecialistScheduleIntervalIdRow,
    MySqlSpecialistScheduleIntervalScheduleIdRow,
    MySqlSpecialistScheduleIntervalStartsAtRow,
    MySqlSpecialistScheduleIntervalWeekdayRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlSpecialistScheduleIntervalRow {
    pub id: MySqlSpecialistScheduleIntervalIdRow,
    pub schedule_id: MySqlSpecialistScheduleIntervalScheduleIdRow,
    pub weekday: MySqlSpecialistScheduleIntervalWeekdayRow,
    pub starts_at: MySqlSpecialistScheduleIntervalStartsAtRow,
    pub ends_at: MySqlSpecialistScheduleIntervalEndsAtRow
}

impl TryFrom<MySqlSpecialistScheduleIntervalRow> for SpecialistScheduleInterval {
    type Error = anyhow::Error;

    fn try_from(row: MySqlSpecialistScheduleIntervalRow) -> Result<Self, Self::Error> {
        SpecialistScheduleInterval::restore(
            uuid::Uuid::from(row.id).into(),
            uuid::Uuid::from(row.schedule_id).into(),
            u8::from(row.weekday).try_into()?,
            chrono::NaiveTime::from(row.starts_at).into(),
            chrono::NaiveTime::from(row.ends_at).into()
        )
    }
}

impl From<&SpecialistScheduleInterval> for MySqlSpecialistScheduleIntervalRow {
    fn from(entity: &SpecialistScheduleInterval) -> Self {
        Self {
            id: uuid::Uuid::from(entity.id()).into(),
            schedule_id: uuid::Uuid::from(entity.schedule_id()).into(),
            weekday: u8::from(entity.weekday()).into(),
            starts_at: chrono::NaiveTime::from(entity.starts_at()).into(),
            ends_at: chrono::NaiveTime::from(entity.ends_at()).into()
        }
    }
}
