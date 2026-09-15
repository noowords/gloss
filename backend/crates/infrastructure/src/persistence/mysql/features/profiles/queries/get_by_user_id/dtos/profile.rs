use application::features::account::queries::get_profile::dtos::{ Profile };

use crate::persistence::mysql::features::profiles::rows::value_objects::{ MySqlProfileFirstNameRow, MySqlProfileLastNameRow, MySqlProfileAvatarUrlRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlProfileRow {
    pub first_name: MySqlProfileFirstNameRow,
    pub last_name: Option<MySqlProfileLastNameRow>,
    pub avatar_url: Option<MySqlProfileAvatarUrlRow>
}

impl From<MySqlProfileRow> for Profile {
    fn from(row: MySqlProfileRow) -> Self {
        Self {
            first_name: row.first_name.into(),
            last_name: row.last_name.map(|v| v.into()),
            avatar_url: row.avatar_url.map(|v| v.into()),
            bio: None
        }
    }
}

impl From<&Profile> for MySqlProfileRow {
    fn from(entity: &Profile) -> Self {
        Self {
            first_name: entity.first_name.clone().into(),
            last_name: entity.last_name.clone().map(|v| v.into()),
            avatar_url: entity.avatar_url.clone().map(|v| v.into())
        }
    }
}
