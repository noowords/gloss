#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistStatus(String);

impl From<SpecialistStatus> for String {
    fn from(value: SpecialistStatus) -> Self {
        value.0
    }
}

impl TryFrom<String> for SpecialistStatus {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 32 {
            anyhow::bail!("Invalid SpecialistStatus");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for SpecialistStatus {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
