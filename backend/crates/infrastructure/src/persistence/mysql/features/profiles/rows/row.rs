use domain::aggregates::users::profile::{ Profile };

use crate::persistence::mysql::features::users::rows::value_objects::{ MySqlUserIdRow };

use super::value_objects::{ MySqlProfileFirstNameRow, MySqlProfileLastNameRow, MySqlProfileAvatarUrlRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlProfileRow {
    pub user_id: MySqlUserIdRow,
    pub first_name: MySqlProfileFirstNameRow,
    pub last_name: Option<MySqlProfileLastNameRow>,
    pub avatar_url: Option<MySqlProfileAvatarUrlRow>
}

impl From<MySqlProfileRow> for Profile {
    fn from(row: MySqlProfileRow) -> Self {
        Self::restore(
            row.user_id.into(),
            row.first_name.into(),
            row.last_name.map(|ln| ln.into()),
            row.avatar_url.map(|au| au.into())
        ).expect("Invalid profile row")
    }
}

impl From<&Profile> for MySqlProfileRow {
    fn from(entity: &Profile) -> Self {
        Self {
            user_id: entity.user_id().into(),
            first_name: entity.first_name().into(),
            last_name: entity.last_name().map(|ln| ln.into()),
            avatar_url: entity.avatar_url().map(|au| au.into())
        }
    }
}
