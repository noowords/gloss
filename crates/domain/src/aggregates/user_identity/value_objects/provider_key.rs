#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserIdentityProviderKey(String);

impl From<UserIdentityProviderKey> for String {
    fn from(provider_key: UserIdentityProviderKey) -> Self {
        provider_key.0
    }
}

impl From<String> for UserIdentityProviderKey {
    fn from(str: String) -> Self {
        Self(str)
    }
}

impl From<&str> for UserIdentityProviderKey {
    fn from(str: &str) -> Self {
        Self(str.to_string())
    }
}
