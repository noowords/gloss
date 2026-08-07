use domain::aggregates::user::value_objects::{ UserRole };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlUserRoleRow(String);

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
