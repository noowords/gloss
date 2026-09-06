use domain::aggregates::profile::value_objects::{ ProfileAvatarUrl };

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
        row.0.into()
    }
}

impl From<ProfileAvatarUrl> for MySqlProfileAvatarUrlRow {
    fn from(entity: ProfileAvatarUrl) -> Self {
        Self(entity.into())
    }
}
