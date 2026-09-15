use domain::aggregates::specialists::specialist_schedule_override::SpecialistScheduleOverride;

use super::value_objects::{
    MySqlSpecialistScheduleOverrideDateRow,
    MySqlSpecialistScheduleOverrideIdRow,
    MySqlSpecialistScheduleOverrideReasonRow,
    MySqlSpecialistScheduleOverrideSpecialistIdRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlSpecialistScheduleOverrideRow {
    pub id: MySqlSpecialistScheduleOverrideIdRow,
    pub specialist_id: MySqlSpecialistScheduleOverrideSpecialistIdRow,
    pub date: MySqlSpecialistScheduleOverrideDateRow,
    pub reason: Option<MySqlSpecialistScheduleOverrideReasonRow>
}

impl TryFrom<MySqlSpecialistScheduleOverrideRow> for SpecialistScheduleOverride {
    type Error = anyhow::Error;

    fn try_from(row: MySqlSpecialistScheduleOverrideRow) -> Result<Self, Self::Error> {
        SpecialistScheduleOverride::restore(
            uuid::Uuid::from(row.id).into(),
            uuid::Uuid::from(row.specialist_id).into(),
            chrono::NaiveDate::from(row.date).into(),
            row.reason.map(|value| String::from(value).try_into()).transpose()?
        )
    }
}

impl From<&SpecialistScheduleOverride> for MySqlSpecialistScheduleOverrideRow {
    fn from(entity: &SpecialistScheduleOverride) -> Self {
        Self {
            id: uuid::Uuid::from(entity.id()).into(),
            specialist_id: uuid::Uuid::from(entity.specialist_id()).into(),
            date: chrono::NaiveDate::from(entity.date()).into(),
            reason: entity.reason().map(|value| String::from(value).into())
        }
    }
}
