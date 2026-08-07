use uuid::{ Uuid };

use domain::aggregates::user_identity::value_objects::{ UserIdentityId };

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlUserIdentityIdRow(Uuid);

impl From<MySqlUserIdentityIdRow> for UserIdentityId {
    fn from(row: MySqlUserIdentityIdRow) -> Self {
        row.0.into()
    }
}

impl From<UserIdentityId> for MySqlUserIdentityIdRow {
    fn from(entity: UserIdentityId) -> Self {
        Self(entity.into())
    }
}
