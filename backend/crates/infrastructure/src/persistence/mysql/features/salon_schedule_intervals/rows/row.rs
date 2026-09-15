use domain::aggregates::salons::salon_schedule_interval::SalonScheduleInterval;

use super::value_objects::{
    MySqlSalonScheduleIntervalEndsAtRow,
    MySqlSalonScheduleIntervalIdRow,
    MySqlSalonScheduleIntervalSalonIdRow,
    MySqlSalonScheduleIntervalStartsAtRow,
    MySqlSalonScheduleIntervalWeekdayRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlSalonScheduleIntervalRow {
    pub id: MySqlSalonScheduleIntervalIdRow,
    pub salon_id: MySqlSalonScheduleIntervalSalonIdRow,
    pub weekday: MySqlSalonScheduleIntervalWeekdayRow,
    pub starts_at: MySqlSalonScheduleIntervalStartsAtRow,
    pub ends_at: MySqlSalonScheduleIntervalEndsAtRow
}

impl TryFrom<MySqlSalonScheduleIntervalRow> for SalonScheduleInterval {
    type Error = anyhow::Error;

    fn try_from(row: MySqlSalonScheduleIntervalRow) -> Result<Self, Self::Error> {
        SalonScheduleInterval::restore(
            uuid::Uuid::from(row.id).into(),
            uuid::Uuid::from(row.salon_id).into(),
            u8::from(row.weekday).try_into()?,
            chrono::NaiveTime::from(row.starts_at).into(),
            chrono::NaiveTime::from(row.ends_at).into()
        )
    }
}

impl From<&SalonScheduleInterval> for MySqlSalonScheduleIntervalRow {
    fn from(entity: &SalonScheduleInterval) -> Self {
        Self {
            id: uuid::Uuid::from(entity.id()).into(),
            salon_id: uuid::Uuid::from(entity.salon_id()).into(),
            weekday: u8::from(entity.weekday()).into(),
            starts_at: chrono::NaiveTime::from(entity.starts_at()).into(),
            ends_at: chrono::NaiveTime::from(entity.ends_at()).into()
        }
    }
}
