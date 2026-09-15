#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewStatus(String);

impl From<ReviewStatus> for String {
    fn from(value: ReviewStatus) -> Self {
        value.0
    }
}

impl TryFrom<String> for ReviewStatus {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 32 {
            anyhow::bail!("Invalid ReviewStatus");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for ReviewStatus {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
