#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UserIdentityProviderType {
    Phone
}

impl From<UserIdentityProviderType> for String {
    fn from(provider_type: UserIdentityProviderType) -> Self {
        match provider_type {
            UserIdentityProviderType::Phone => "phone"
        }.to_string()
    }
}

impl TryFrom<String> for UserIdentityProviderType {
    type Error = anyhow::Error;
    
    fn try_from(str: String) -> Result<Self, Self::Error> {
        match str.as_str() {
            "phone" => Some(UserIdentityProviderType::Phone),
            _ => None
        }.ok_or_else(|| anyhow::anyhow!("Invalid UserIdentityProviderType: {}", str.to_string()))
    }
}

impl TryFrom<&str> for UserIdentityProviderType {
    type Error = anyhow::Error;
    
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str {
            "phone" => Some(UserIdentityProviderType::Phone),
            _ => None
        }.ok_or_else(|| anyhow::anyhow!("Invalid UserIdentityProviderType: {}", str.to_string()))
    }
}
