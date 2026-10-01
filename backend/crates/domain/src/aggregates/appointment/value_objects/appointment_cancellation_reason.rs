use super::super::errors::AppointmentError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppointmentCancellationReason(String);

// MARK: Conversions
impl TryFrom<String> for AppointmentCancellationReason {
    type Error = AppointmentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(AppointmentError::CancellationReasonEmpty);
        }

        if value.chars().count() > 255 {
            return Err(AppointmentError::CancellationReasonTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<AppointmentCancellationReason> for String {
    fn from(vo: AppointmentCancellationReason) -> Self {
        vo.0
    }
}
