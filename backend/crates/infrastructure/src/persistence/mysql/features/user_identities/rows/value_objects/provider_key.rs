use domain::aggregates::user_identity::value_objects::{ UserIdentityProviderKey };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlUserIdentityProviderKeyRow(String);

impl From<MySqlUserIdentityProviderKeyRow> for String {
    fn from(row: MySqlUserIdentityProviderKeyRow) -> Self {
        row.0
    }
}

impl From<String> for MySqlUserIdentityProviderKeyRow {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<MySqlUserIdentityProviderKeyRow> for UserIdentityProviderKey {
    fn from(row: MySqlUserIdentityProviderKeyRow) -> Self {
        row.0.into()
    }
}

impl From<UserIdentityProviderKey> for MySqlUserIdentityProviderKeyRow {
    fn from(entity: UserIdentityProviderKey) -> Self {
        Self(entity.into())
    }
}
