use serde_json::{ Value };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserIdentityProviderData(Value);

impl From<Value> for UserIdentityProviderData {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

impl From<UserIdentityProviderData> for Value {
    fn from(provider_data: UserIdentityProviderData) -> Self {
        provider_data.0
    }
}

impl TryFrom<String> for UserIdentityProviderData {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed: Value = serde_json::from_str(&value)?;
        
        Ok(Self(parsed))
    }
}

impl TryFrom<&str> for UserIdentityProviderData {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed: Value = serde_json::from_str(value)?;
        
        Ok(Self(parsed))
    }
}

impl From<UserIdentityProviderData> for String {
    fn from(provider_data: UserIdentityProviderData) -> Self {
        provider_data.0.to_string()
    }
}
