use domain::aggregates::profile::value_objects::{ ProfileAvatarUrl };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlProfileAvatarUrlRow(String);

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
