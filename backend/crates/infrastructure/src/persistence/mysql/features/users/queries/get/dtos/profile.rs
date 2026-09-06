use application::features::users::queries::get::dtos::{ Profile };

use crate::persistence::mysql::features::profiles::rows::value_objects::{ MySqlProfileFirstNameRow, MySqlProfileLastNameRow, MySqlProfileAvatarUrlRow, MySqlProfileBioRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlProfileRow {
    pub first_name: MySqlProfileFirstNameRow,
    pub last_name: Option<MySqlProfileLastNameRow>,
    pub avatar_url: Option<MySqlProfileAvatarUrlRow>,
    pub bio: Option<MySqlProfileBioRow>
}

impl From<MySqlProfileRow> for Profile {
    fn from(row: MySqlProfileRow) -> Self {
        Self {
            first_name: row.first_name.into(),
            last_name: row.last_name.map(|v| v.into()),
            avatar_url: row.avatar_url.map(|v| v.into()),
            bio: row.bio.map(|v| v.into())
        }
    }
}

impl From<&Profile> for MySqlProfileRow {
    fn from(entity: &Profile) -> Self {
        Self {
            first_name: entity.first_name.clone().into(),
            last_name: entity.last_name.clone().map(|v| v.into()),
            avatar_url: entity.avatar_url.clone().map(|v| v.into()),
            bio: entity.bio.clone().map(|v| v.into())
        }
    }
}
