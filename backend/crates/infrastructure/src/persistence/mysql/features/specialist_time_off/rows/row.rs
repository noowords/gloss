use domain::aggregates::specialists::specialist_time_off::SpecialistTimeOff;

use super::value_objects::{
    MySqlSpecialistTimeOffChargedLeaveMinutesRow,
    MySqlSpecialistTimeOffDateRow,
    MySqlSpecialistTimeOffEndsAtRow,
    MySqlSpecialistTimeOffIdRow,
    MySqlSpecialistTimeOffReasonRow,
    MySqlSpecialistTimeOffSpecialistIdRow,
    MySqlSpecialistTimeOffStartsAtRow,
    MySqlSpecialistTimeOffTypeRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlSpecialistTimeOffRow {
    pub id: MySqlSpecialistTimeOffIdRow,
    pub specialist_id: MySqlSpecialistTimeOffSpecialistIdRow,
    pub date: MySqlSpecialistTimeOffDateRow,
    pub r#type: MySqlSpecialistTimeOffTypeRow,
    pub starts_at: Option<MySqlSpecialistTimeOffStartsAtRow>,
    pub ends_at: Option<MySqlSpecialistTimeOffEndsAtRow>,
    pub charged_leave_minutes: MySqlSpecialistTimeOffChargedLeaveMinutesRow,
    pub reason: Option<MySqlSpecialistTimeOffReasonRow>
}

impl TryFrom<MySqlSpecialistTimeOffRow> for SpecialistTimeOff {
    type Error = anyhow::Error;

    fn try_from(row: MySqlSpecialistTimeOffRow) -> Result<Self, Self::Error> {
        SpecialistTimeOff::restore(
            uuid::Uuid::from(row.id).into(),
            uuid::Uuid::from(row.specialist_id).into(),
            chrono::NaiveDate::from(row.date).into(),
            String::from(row.r#type).try_into()?,
            row.starts_at.map(|value| chrono::NaiveTime::from(value).into()),
            row.ends_at.map(|value| chrono::NaiveTime::from(value).into()),
            u16::from(row.charged_leave_minutes).try_into()?,
            row.reason.map(|value| String::from(value).try_into()).transpose()?
        )
    }
}

impl From<&SpecialistTimeOff> for MySqlSpecialistTimeOffRow {
    fn from(entity: &SpecialistTimeOff) -> Self {
        Self {
            id: uuid::Uuid::from(entity.id()).into(),
            specialist_id: uuid::Uuid::from(entity.specialist_id()).into(),
            date: chrono::NaiveDate::from(entity.date()).into(),
            r#type: String::from(entity.r#type()).into(),
            starts_at: entity.starts_at().map(|value| chrono::NaiveTime::from(value).into()),
            ends_at: entity.ends_at().map(|value| chrono::NaiveTime::from(value).into()),
            charged_leave_minutes: u16::from(entity.charged_leave_minutes()).into(),
            reason: entity.reason().map(|value| String::from(value).into())
        }
    }
}
