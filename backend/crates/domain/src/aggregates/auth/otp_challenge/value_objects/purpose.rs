#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtpChallengePurpose(String);

impl From<OtpChallengePurpose> for String {
    fn from(value: OtpChallengePurpose) -> Self {
        value.0
    }
}

impl TryFrom<String> for OtpChallengePurpose {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 32 {
            anyhow::bail!("Invalid OtpChallengePurpose");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for OtpChallengePurpose {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
