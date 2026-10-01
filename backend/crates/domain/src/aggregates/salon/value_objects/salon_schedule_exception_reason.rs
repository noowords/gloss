use super::super::errors::SalonScheduleExceptionError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonScheduleExceptionReason(String);

// MARK: Conversions
impl TryFrom<String> for SalonScheduleExceptionReason {
    type Error = SalonScheduleExceptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(SalonScheduleExceptionError::ReasonEmpty);
        }

        if value.chars().count() > 255 {
            return Err(SalonScheduleExceptionError::ReasonTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<SalonScheduleExceptionReason> for String {
    fn from(vo: SalonScheduleExceptionReason) -> Self {
        vo.0
    }
}
