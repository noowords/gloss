use domain::aggregates::users::user_role::UserRole;

use super::value_objects::{ MySqlUserRoleRoleRow, MySqlUserRoleUserIdRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlUserRoleRow {
    pub user_id: MySqlUserRoleUserIdRow,
    pub role: MySqlUserRoleRoleRow
}

impl TryFrom<MySqlUserRoleRow> for UserRole {
    type Error = anyhow::Error;

    fn try_from(row: MySqlUserRoleRow) -> Result<Self, Self::Error> {
        UserRole::restore(uuid::Uuid::from(row.user_id).into(), String::from(row.role).try_into()?)
    }
}

impl From<&UserRole> for MySqlUserRoleRow {
    fn from(entity: &UserRole) -> Self {
        Self {
            user_id: uuid::Uuid::from(entity.user_id()).into(),
            role: String::from(entity.role()).into()
        }
    }
}
