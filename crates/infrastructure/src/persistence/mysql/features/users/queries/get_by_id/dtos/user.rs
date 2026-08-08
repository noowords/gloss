use uuid::{ Uuid };

use application::features::users::queries::get_by_id::dtos::{ User };

use super::{ MySqlProfileRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlUserRow {
    pub id: Uuid,
    pub role: String,

    #[sqlx(flatten)]
    pub profile: MySqlProfileRow
}

impl From<MySqlUserRow> for User {
    fn from(row: MySqlUserRow) -> Self {
        Self {
            id: row.id,
            role: row.role,
            profile: row.profile.into()
        }
    }
}

impl From<&User> for MySqlUserRow {
    fn from(entity: &User) -> Self {
        Self {
            id: entity.id,
            role: entity.role.clone(),
            profile: MySqlProfileRow {
                first_name: entity.profile.first_name.clone(),
                last_name: entity.profile.last_name.clone(),
                avatar_url: entity.profile.avatar_url.clone(),
                bio: entity.profile.bio.clone()
            }
        }
    }
}
