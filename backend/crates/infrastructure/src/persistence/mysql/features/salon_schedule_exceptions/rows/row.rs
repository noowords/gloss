use domain::aggregates::salons::salon_schedule_exception::SalonScheduleException;

use super::value_objects::{
    MySqlSalonScheduleExceptionDateRow,
    MySqlSalonScheduleExceptionIdRow,
    MySqlSalonScheduleExceptionReasonRow,
    MySqlSalonScheduleExceptionSalonIdRow,
    MySqlSalonScheduleExceptionTypeRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlSalonScheduleExceptionRow {
    pub id: MySqlSalonScheduleExceptionIdRow,
    pub salon_id: MySqlSalonScheduleExceptionSalonIdRow,
    pub date: MySqlSalonScheduleExceptionDateRow,
    pub r#type: MySqlSalonScheduleExceptionTypeRow,
    pub reason: Option<MySqlSalonScheduleExceptionReasonRow>
}

impl TryFrom<MySqlSalonScheduleExceptionRow> for SalonScheduleException {
    type Error = anyhow::Error;

    fn try_from(row: MySqlSalonScheduleExceptionRow) -> Result<Self, Self::Error> {
        SalonScheduleException::restore(
            uuid::Uuid::from(row.id).into(),
            uuid::Uuid::from(row.salon_id).into(),
            chrono::NaiveDate::from(row.date).into(),
            String::from(row.r#type).try_into()?,
            row.reason.map(|value| String::from(value).try_into()).transpose()?
        )
    }
}

impl From<&SalonScheduleException> for MySqlSalonScheduleExceptionRow {
    fn from(entity: &SalonScheduleException) -> Self {
        Self {
            id: uuid::Uuid::from(entity.id()).into(),
            salon_id: uuid::Uuid::from(entity.salon_id()).into(),
            date: chrono::NaiveDate::from(entity.date()).into(),
            r#type: String::from(entity.r#type()).into(),
            reason: entity.reason().map(|value| String::from(value).into())
        }
    }
}
