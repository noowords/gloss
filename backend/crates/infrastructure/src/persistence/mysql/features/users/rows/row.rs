use domain::aggregates::users::user::{ User };

use super::value_objects::{ MySqlUserIdRow, MySqlUserStatusRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlUserRow {
    pub id: MySqlUserIdRow,
    pub status: MySqlUserStatusRow
}

impl TryFrom<MySqlUserRow> for User {
    type Error = anyhow::Error;

    fn try_from(row: MySqlUserRow) -> Result<Self, Self::Error> {
        Ok(Self::restore(
            row.id.into(),
            row.status.try_into()?
        )?)
    }
}

impl From<&User> for MySqlUserRow {
    fn from(entity: &User) -> Self {
        Self {
            id: entity.id().into(),
            status: entity.status().into()
        }
    }
}
