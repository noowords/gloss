use domain::aggregates::users::profile::value_objects::{ ProfileAvatarUrl };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlProfileAvatarUrlRow(String);

impl From<MySqlProfileAvatarUrlRow> for String {
    fn from(row: MySqlProfileAvatarUrlRow) -> Self {
        row.0
    }
}

impl From<String> for MySqlProfileAvatarUrlRow {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<MySqlProfileAvatarUrlRow> for ProfileAvatarUrl {
    fn from(row: MySqlProfileAvatarUrlRow) -> Self {
        row.0.try_into().expect("Invalid profile avatar url")
    }
}

impl From<ProfileAvatarUrl> for MySqlProfileAvatarUrlRow {
    fn from(entity: ProfileAvatarUrl) -> Self {
        Self(entity.into())
    }
}
