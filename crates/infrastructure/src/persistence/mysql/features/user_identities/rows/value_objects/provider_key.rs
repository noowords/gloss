use domain::aggregates::user_identity::value_objects::{ UserIdentityProviderKey };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlUserIdentityProviderKeyRow(String);

impl From<MySqlUserIdentityProviderKeyRow> for UserIdentityProviderKey {
    fn from(model: MySqlUserIdentityProviderKeyRow) -> Self {
        model.0.into()
    }
}

impl From<UserIdentityProviderKey> for MySqlUserIdentityProviderKeyRow {
    fn from(entity: UserIdentityProviderKey) -> Self {
        Self(entity.into())
    }
}
