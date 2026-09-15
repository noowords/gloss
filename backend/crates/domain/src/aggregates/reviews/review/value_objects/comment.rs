#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewComment(String);

impl From<ReviewComment> for String {
    fn from(value: ReviewComment) -> Self {
        value.0
    }
}

impl TryFrom<String> for ReviewComment {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 65535 {
            anyhow::bail!("Invalid ReviewComment");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for ReviewComment {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
