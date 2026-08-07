use domain::aggregates::profile::{ Profile };

use crate::persistence::mysql::features::users::rows::value_objects::{ MySqlUserIdRow };

use super::value_objects::{ MySqlProfileFirstNameRow, MySqlProfileLastNameRow, MySqlProfileAvatarUrlRow, MySqlProfileBioRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlProfileRow {
    pub user_id: MySqlUserIdRow,
    pub first_name: MySqlProfileFirstNameRow,
    pub last_name: Option<MySqlProfileLastNameRow>,
    pub avatar_url: Option<MySqlProfileAvatarUrlRow>,
    pub bio: Option<MySqlProfileBioRow>,
}

impl From<MySqlProfileRow> for Profile {
    fn from(model: MySqlProfileRow) -> Self {
        Self::restore(
            model.user_id.into(),
            model.first_name.into(),
            model.last_name.map(|ln| ln.into()),
            model.avatar_url.map(|au| au.into()),
            model.bio.map(|b| b.into())
        )
    }
}

impl From<&Profile> for MySqlProfileRow {
    fn from(entity: &Profile) -> Self {
        Self {
            user_id: entity.user_id().into(),
            first_name: entity.first_name().into(),
            last_name: entity.last_name().map(|ln| ln.into()),
            avatar_url: entity.avatar_url().map(|au| au.into()),
            bio: entity.bio().map(|b| b.into())
        }
    }
}
