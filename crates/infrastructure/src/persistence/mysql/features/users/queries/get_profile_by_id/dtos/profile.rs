use application::features::users::queries::get_profile_by_id::dtos::{ Profile };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlProfileRow {
    pub first_name: String,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>
}

impl From<MySqlProfileRow> for Profile {
    fn from(row: MySqlProfileRow) -> Self {
        Self {
            first_name: row.first_name,
            last_name: row.last_name,
            avatar_url: row.avatar_url,
            bio: row.bio
        }
    }
}

impl From<&Profile> for MySqlProfileRow {
    fn from(entity: &Profile) -> Self {
        Self {
            first_name: entity.first_name.clone(),
            last_name: entity.last_name.clone(),
            avatar_url: entity.avatar_url.clone(),
            bio: entity.bio.clone()
        }
    }
}
