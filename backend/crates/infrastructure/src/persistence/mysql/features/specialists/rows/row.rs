use domain::aggregates::specialists::specialist::{ Specialist };

use crate::persistence::mysql::features::users::rows::value_objects::{ MySqlUserIdRow };

use super::value_objects::{
    MySqlSpecialistBioRow,
    MySqlSpecialistExperienceStartedAtRow,
    MySqlSpecialistIdRow,
    MySqlSpecialistSalonIdRow,
    MySqlSpecialistStatusRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlSpecialistRow {
    pub id: MySqlSpecialistIdRow,
    pub user_id: MySqlUserIdRow,
    pub salon_id: MySqlSpecialistSalonIdRow,
    pub bio: Option<MySqlSpecialistBioRow>,
    pub experience_started_at: Option<MySqlSpecialistExperienceStartedAtRow>,
    pub status: MySqlSpecialistStatusRow
}

impl TryFrom<MySqlSpecialistRow> for Specialist {
    type Error = anyhow::Error;

    fn try_from(row: MySqlSpecialistRow) -> Result<Self, Self::Error> {
        Self::restore(
            uuid::Uuid::from(row.id).into(),
            row.user_id.into(),
            uuid::Uuid::from(row.salon_id).into(),
            row.bio.map(|value| String::from(value).try_into()).transpose()?,
            row.experience_started_at.map(|value| chrono::NaiveDate::from(value).into()),
            String::from(row.status).try_into()?
        )
    }
}

impl From<&Specialist> for MySqlSpecialistRow {
    fn from(entity: &Specialist) -> Self {
        Self {
            id: uuid::Uuid::from(entity.id()).into(),
            user_id: entity.user_id().into(),
            salon_id: uuid::Uuid::from(entity.salon_id()).into(),
            bio: entity.bio().map(|value| String::from(value).into()),
            experience_started_at: entity.experience_started_at().map(|value| chrono::NaiveDate::from(value).into()),
            status: String::from(entity.status()).into()
        }
    }
}
