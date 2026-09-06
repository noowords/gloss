use application::features::specialists::queries::get_by_user_id::dtos::{ Specialist };

use crate::persistence::mysql::features::users::rows::value_objects::{ MySqlUserIdRow };

use super::{ MySqlProfileRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlSpecialistRow {
    pub user_id: MySqlUserIdRow,

    #[sqlx(flatten)]
    pub profile: MySqlProfileRow
}

impl From<MySqlSpecialistRow> for Specialist {
    fn from(row: MySqlSpecialistRow) -> Self {
        Self {
            user_id: row.user_id.into(),
            profile: row.profile.into()
        }
    }
}

impl From<&Specialist> for MySqlSpecialistRow {
    fn from(entity: &Specialist) -> Self {
        Self {
            user_id: entity.user_id.into(),
            profile: MySqlProfileRow {
                first_name: entity.profile.first_name.clone().into(),
                last_name: entity.profile.last_name.clone().map(|v| v.into()),
                avatar_url: entity.profile.avatar_url.clone().map(|v| v.into()),
                bio: entity.profile.bio.clone().map(|v| v.into())
            }
        }
    }
}
