use super::super::errors::SpecialistScheduleOverrideError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleOverrideReason(String);

// MARK: Conversions
impl TryFrom<String> for SpecialistScheduleOverrideReason {
    type Error = SpecialistScheduleOverrideError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(SpecialistScheduleOverrideError::ReasonEmpty);
        }

        if value.chars().count() > 255 {
            return Err(SpecialistScheduleOverrideError::ReasonTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<SpecialistScheduleOverrideReason> for String {
    fn from(vo: SpecialistScheduleOverrideReason) -> Self {
        vo.0
    }
}
