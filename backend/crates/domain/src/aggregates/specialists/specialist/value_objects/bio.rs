#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistBio(String);

impl From<SpecialistBio> for String {
    fn from(value: SpecialistBio) -> Self {
        value.0
    }
}

impl TryFrom<String> for SpecialistBio {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 65535 {
            anyhow::bail!("Invalid SpecialistBio");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for SpecialistBio {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
