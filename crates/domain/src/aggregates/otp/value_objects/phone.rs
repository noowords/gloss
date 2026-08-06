#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtpPhone(String);

impl OtpPhone {
    pub fn new(value: String) -> Result<Self, anyhow::Error> {
        if value.is_empty() {
            return Err(anyhow::anyhow!("Phone number cannot be empty"));
        }
        
        if !value.chars().all(|c| c.is_digit(10)) {
            return Err(anyhow::anyhow!("Phone number must contain only digits"));
        }
        
        Ok(Self(value))
    }
}

impl From<OtpPhone> for String {
    fn from(phone: OtpPhone) -> Self {
        phone.0
    }
}

impl TryFrom<String> for OtpPhone {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for OtpPhone {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_string())
    }
}
