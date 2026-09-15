#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtpChallengeSubject(String);

impl From<OtpChallengeSubject> for String {
    fn from(value: OtpChallengeSubject) -> Self {
        value.0
    }
}

impl TryFrom<String> for OtpChallengeSubject {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 255 {
            anyhow::bail!("Invalid OtpChallengeSubject");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for OtpChallengeSubject {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
