#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtpChallengeProvider(String);

impl From<OtpChallengeProvider> for String {
    fn from(value: OtpChallengeProvider) -> Self {
        value.0
    }
}

impl TryFrom<String> for OtpChallengeProvider {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 32 {
            anyhow::bail!("Invalid OtpChallengeProvider");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for OtpChallengeProvider {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
