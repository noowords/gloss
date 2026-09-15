use domain::aggregates::specialists::specialist_schedule_override_interval::SpecialistScheduleOverrideInterval;

use super::value_objects::{
    MySqlSpecialistScheduleOverrideIntervalEndsAtRow,
    MySqlSpecialistScheduleOverrideIntervalIdRow,
    MySqlSpecialistScheduleOverrideIntervalOverrideIdRow,
    MySqlSpecialistScheduleOverrideIntervalStartsAtRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlSpecialistScheduleOverrideIntervalRow {
    pub id: MySqlSpecialistScheduleOverrideIntervalIdRow,
    pub override_id: MySqlSpecialistScheduleOverrideIntervalOverrideIdRow,
    pub starts_at: MySqlSpecialistScheduleOverrideIntervalStartsAtRow,
    pub ends_at: MySqlSpecialistScheduleOverrideIntervalEndsAtRow
}

impl TryFrom<MySqlSpecialistScheduleOverrideIntervalRow> for SpecialistScheduleOverrideInterval {
    type Error = anyhow::Error;

    fn try_from(row: MySqlSpecialistScheduleOverrideIntervalRow) -> Result<Self, Self::Error> {
        SpecialistScheduleOverrideInterval::restore(
            uuid::Uuid::from(row.id).into(),
            uuid::Uuid::from(row.override_id).into(),
            chrono::NaiveTime::from(row.starts_at).into(),
            chrono::NaiveTime::from(row.ends_at).into()
        )
    }
}

impl From<&SpecialistScheduleOverrideInterval> for MySqlSpecialistScheduleOverrideIntervalRow {
    fn from(entity: &SpecialistScheduleOverrideInterval) -> Self {
        Self {
            id: uuid::Uuid::from(entity.id()).into(),
            override_id: uuid::Uuid::from(entity.override_id()).into(),
            starts_at: chrono::NaiveTime::from(entity.starts_at()).into(),
            ends_at: chrono::NaiveTime::from(entity.ends_at()).into()
        }
    }
}
