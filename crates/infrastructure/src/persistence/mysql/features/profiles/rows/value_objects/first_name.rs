use domain::aggregates::profile::value_objects::{ ProfileFirstName };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlProfileFirstNameRow(String);

impl From<MySqlProfileFirstNameRow> for String {
    fn from(row: MySqlProfileFirstNameRow) -> Self {
        row.0
    }
}

impl From<String> for MySqlProfileFirstNameRow {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<MySqlProfileFirstNameRow> for ProfileFirstName {
    fn from(row: MySqlProfileFirstNameRow) -> Self {
        row.0.into()
    }
}

impl From<ProfileFirstName> for MySqlProfileFirstNameRow {
    fn from(entity: ProfileFirstName) -> Self {
        Self(entity.into())
    }
}
