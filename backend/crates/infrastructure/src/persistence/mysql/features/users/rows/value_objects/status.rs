use domain::aggregates::users::user::value_objects::{ UserStatus };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlUserStatusRow(String);

impl From<MySqlUserStatusRow> for String {
    fn from(row: MySqlUserStatusRow) -> Self {
        row.0
    }
}

impl From<String> for MySqlUserStatusRow {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl TryFrom<MySqlUserStatusRow> for UserStatus {
    type Error = anyhow::Error;

    fn try_from(row: MySqlUserStatusRow) -> Result<Self, Self::Error> {
        row.0.try_into()
    }
}

impl From<UserStatus> for MySqlUserStatusRow {
    fn from(entity: UserStatus) -> Self {
        Self(entity.into())
    }
}
