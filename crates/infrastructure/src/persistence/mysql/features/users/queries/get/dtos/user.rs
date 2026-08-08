use application::features::users::queries::get::dtos::{ User };

use crate::persistence::mysql::features::users::rows::value_objects::{ MySqlUserIdRow, MySqlUserRoleRow };

use super::{ MySqlProfileRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlUserRow {
    pub id: MySqlUserIdRow,
    pub role: MySqlUserRoleRow,

    #[sqlx(flatten)]
    pub profile: MySqlProfileRow
}

impl From<MySqlUserRow> for User {
    fn from(row: MySqlUserRow) -> Self {
        Self {
            id: row.id.into(),
            role: row.role.into(),
            profile: row.profile.into()
        }
    }
}

impl From<&User> for MySqlUserRow {
    fn from(entity: &User) -> Self {
        Self {
            id: entity.id.into(),
            role: entity.role.clone().into(),
            profile: MySqlProfileRow {
                first_name: entity.profile.first_name.clone().into(),
                last_name: entity.profile.last_name.clone().map(|v| v.into()),
                avatar_url: entity.profile.avatar_url.clone().map(|v| v.into()),
                bio: entity.profile.bio.clone().map(|v| v.into())
            }
        }
    }
}
