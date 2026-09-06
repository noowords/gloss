#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OtpProviderType {
    Phone
}

impl From<OtpProviderType> for String {
    fn from(provider_type: OtpProviderType) -> Self {
        match provider_type {
            OtpProviderType::Phone => "phone"
        }.to_string()
    }
}

impl TryFrom<String> for OtpProviderType {
    type Error = anyhow::Error;
    
    fn try_from(str: String) -> Result<Self, Self::Error> {
        match str.as_str() {
            "phone" => Some(OtpProviderType::Phone),
            _ => None
        }.ok_or_else(|| anyhow::anyhow!("Invalid OtpProviderType: {}", str.to_string()))
    }
}

impl TryFrom<&str> for OtpProviderType {
    type Error = anyhow::Error;
    
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str {
            "phone" => Some(OtpProviderType::Phone),
            _ => None
        }.ok_or_else(|| anyhow::anyhow!("Invalid OtpProviderType: {}", str.to_string()))
    }
}
