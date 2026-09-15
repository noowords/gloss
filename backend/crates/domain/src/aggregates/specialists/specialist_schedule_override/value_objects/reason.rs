#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleOverrideReason(String);

impl From<SpecialistScheduleOverrideReason> for String {
    fn from(value: SpecialistScheduleOverrideReason) -> Self {
        value.0
    }
}

impl TryFrom<String> for SpecialistScheduleOverrideReason {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 255 {
            anyhow::bail!("Invalid SpecialistScheduleOverrideReason");
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for SpecialistScheduleOverrideReason {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.to_string().try_into()
    }
}
