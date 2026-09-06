#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserIdentityProviderData {
    Phone {
        number: String
    },
    Telegram {
        telegram_id: u64,
        username: Option<String>
    }
}

impl From<UserIdentityProviderData> for String {
    fn from(provider_data: UserIdentityProviderData) -> Self {
        match provider_data {
            UserIdentityProviderData::Phone { number } => {
                format!("{{\"type\":\"phone\",\"number\":\"{}\"}}", number)
            }
            UserIdentityProviderData::Telegram { telegram_id, username } => {
                format!(
                    "{{\"type\":\"telegram\",\"telegram_id\":{},\"username\":{}}}", 
                    telegram_id,
                    username
                        .map(|u| format!("\"{}\"", u))
                        .unwrap_or_else(|| "null".to_string())
                )
            }
        }
    }
}

impl TryFrom<String> for UserIdentityProviderData {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl TryFrom<&str> for UserIdentityProviderData {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.contains("\"type\":\"phone\"") {
            let phone = s.split("\"number\":\"")
                .nth(1)
                .and_then(|p| p.split('"').next())
                .ok_or_else(|| anyhow::anyhow!("Invalid phone data format in provider string"))?;

            Ok(Self::Phone { number: phone.to_string() })
        } else if s.contains("\"type\":\"telegram\"") {
            let id_str = s.split("\"telegram_id\":")
                .nth(1)
                .and_then(|p| p.split(',').next())
                .and_then(|p| p.split('}').next())
                .ok_or_else(|| anyhow::anyhow!("Invalid telegram_id format in provider string"))?;
            
            let telegram_id = id_str.trim().parse::<u64>()?;

            let username = if s.contains("\"username\":null") {
                None
            } else {
                s.split("\"username\":\"")
                    .nth(1)
                    .and_then(|p| p.split('"').next())
                    .map(|u| u.to_string())
            };

            Ok(Self::Telegram { telegram_id, username })
        } else {
            anyhow::bail!("Unknown identity provider type in string")
        }
    }
}
