#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistTimeOffType(String);

impl From<SpecialistTimeOffType> for String {
    fn from(value: SpecialistTimeOffType) -> Self {
        value.0
    }
}

impl TryFrom<String> for SpecialistTimeOffType {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 32 {
            anyhow::bail!("Invalid SpecialistTimeOffType");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for SpecialistTimeOffType {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
