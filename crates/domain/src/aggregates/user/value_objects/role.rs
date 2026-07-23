#[derive(Clone)]
pub enum UserRole {
    Admin,
    Master,
    Client
}

impl From<UserRole> for String {
    fn from(role: UserRole) -> Self {
        match role {
            UserRole::Admin => "admin",
            UserRole::Master => "master",
            UserRole::Client => "client"
        }.to_string()
    }
}

impl TryFrom<&str> for UserRole {
    type Error = anyhow::Error;
    
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str {
            "admin" => Some(UserRole::Admin),
            "master" => Some(UserRole::Master),
            "client" => Some(UserRole::Client),
            _ => None
        }.ok_or_else(|| anyhow::anyhow!("Invalid UserRole: {}", str.to_string()))
    }
}
