use domain::aggregates::specialists::specialist_schedule::SpecialistSchedule;

use super::value_objects::{
    MySqlSpecialistScheduleEffectiveFromRow,
    MySqlSpecialistScheduleEffectiveUntilRow,
    MySqlSpecialistScheduleIdRow,
    MySqlSpecialistScheduleSpecialistIdRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlSpecialistScheduleRow {
    pub id: MySqlSpecialistScheduleIdRow,
    pub specialist_id: MySqlSpecialistScheduleSpecialistIdRow,
    pub effective_from: MySqlSpecialistScheduleEffectiveFromRow,
    pub effective_until: Option<MySqlSpecialistScheduleEffectiveUntilRow>
}

impl TryFrom<MySqlSpecialistScheduleRow> for SpecialistSchedule {
    type Error = anyhow::Error;

    fn try_from(row: MySqlSpecialistScheduleRow) -> Result<Self, Self::Error> {
        SpecialistSchedule::restore(
            uuid::Uuid::from(row.id).into(),
            uuid::Uuid::from(row.specialist_id).into(),
            chrono::NaiveDate::from(row.effective_from).into(),
            row.effective_until.map(|value| chrono::NaiveDate::from(value).into())
        )
    }
}

impl From<&SpecialistSchedule> for MySqlSpecialistScheduleRow {
    fn from(entity: &SpecialistSchedule) -> Self {
        Self {
            id: uuid::Uuid::from(entity.id()).into(),
            specialist_id: uuid::Uuid::from(entity.specialist_id()).into(),
            effective_from: chrono::NaiveDate::from(entity.effective_from()).into(),
            effective_until: entity.effective_until().map(|value| chrono::NaiveDate::from(value).into())
        }
    }
}
