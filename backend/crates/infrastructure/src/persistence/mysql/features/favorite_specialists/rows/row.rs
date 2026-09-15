use domain::aggregates::users::favorite_specialist::FavoriteSpecialist;

use super::value_objects::{ MySqlFavoriteSpecialistSpecialistIdRow, MySqlFavoriteSpecialistUserIdRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlFavoriteSpecialistRow {
    pub user_id: MySqlFavoriteSpecialistUserIdRow,
    pub specialist_id: MySqlFavoriteSpecialistSpecialistIdRow
}

impl TryFrom<MySqlFavoriteSpecialistRow> for FavoriteSpecialist {
    type Error = anyhow::Error;

    fn try_from(row: MySqlFavoriteSpecialistRow) -> Result<Self, Self::Error> {
        FavoriteSpecialist::restore(uuid::Uuid::from(row.user_id).into(), uuid::Uuid::from(row.specialist_id).into())
    }
}

impl From<&FavoriteSpecialist> for MySqlFavoriteSpecialistRow {
    fn from(entity: &FavoriteSpecialist) -> Self {
        Self {
            user_id: uuid::Uuid::from(entity.user_id()).into(),
            specialist_id: uuid::Uuid::from(entity.specialist_id()).into()
        }
    }
}
