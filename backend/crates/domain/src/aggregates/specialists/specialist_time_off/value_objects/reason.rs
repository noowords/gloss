#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistTimeOffReason(String);

impl From<SpecialistTimeOffReason> for String {
    fn from(value: SpecialistTimeOffReason) -> Self {
        value.0
    }
}

impl TryFrom<String> for SpecialistTimeOffReason {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 255 {
            anyhow::bail!("Invalid SpecialistTimeOffReason");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for SpecialistTimeOffReason {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
