#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UserRoleName {
    Admin,
    Master,
    Client
}

impl From<UserRoleName> for String {
    fn from(role: UserRoleName) -> Self {
        match role {
            UserRoleName::Admin => "admin",
            UserRoleName::Master => "master",
            UserRoleName::Client => "client"
        }.to_string()
    }
}

impl TryFrom<String> for UserRoleName {
    type Error = anyhow::Error;
    
    fn try_from(str: String) -> Result<Self, Self::Error> {
        match str.as_str() {
            "admin" => Some(UserRoleName::Admin),
            "master" => Some(UserRoleName::Master),
            "client" => Some(UserRoleName::Client),
            _ => None
        }.ok_or_else(|| anyhow::anyhow!("Invalid UserRoleName: {}", str.to_string()))
    }
}

impl TryFrom<&str> for UserRoleName {
    type Error = anyhow::Error;
    
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str {
            "admin" => Some(UserRoleName::Admin),
            "master" => Some(UserRoleName::Master),
            "client" => Some(UserRoleName::Client),
            _ => None
        }.ok_or_else(|| anyhow::anyhow!("Invalid UserRoleName: {}", str.to_string()))
    }
}
