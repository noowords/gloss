use domain::aggregates::user::value_objects::{ UserRole };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlUserRoleRow(String);

impl From<MySqlUserRoleRow> for String {
    fn from(row: MySqlUserRoleRow) -> Self {
        row.0
    }
}

impl From<String> for MySqlUserRoleRow {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl TryFrom<MySqlUserRoleRow> for UserRole {
    type Error = anyhow::Error;

    fn try_from(row: MySqlUserRoleRow) -> Result<Self, Self::Error> {
        row.0.try_into()
    }
}

impl From<UserRole> for MySqlUserRoleRow {
    fn from(entity: UserRole) -> Self {
        Self(entity.into())
    }
}
