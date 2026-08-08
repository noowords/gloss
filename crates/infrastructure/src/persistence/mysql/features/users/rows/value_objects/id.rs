use uuid::{ Uuid };

use domain::aggregates::user::value_objects::{ UserId };

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlUserIdRow(Uuid);

impl From<MySqlUserIdRow> for Uuid {
    fn from(row: MySqlUserIdRow) -> Self {
        row.0
    }
}

impl From<Uuid> for MySqlUserIdRow {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<MySqlUserIdRow> for UserId {
    fn from(row: MySqlUserIdRow) -> Self {
        row.0.into()
    }
}

impl From<UserId> for MySqlUserIdRow {
    fn from(entity: UserId) -> Self {
        Self(entity.into())
    }
}
