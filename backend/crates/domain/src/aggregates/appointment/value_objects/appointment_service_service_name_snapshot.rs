use super::super::errors::AppointmentServiceError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppointmentServiceNameSnapshot(String);

// MARK: Conversions
impl TryFrom<String> for AppointmentServiceNameSnapshot {
    type Error = AppointmentServiceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(AppointmentServiceError::NameSnapshotEmpty);
        }

        if value.chars().count() > 128 {
            return Err(AppointmentServiceError::NameSnapshotTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<AppointmentServiceNameSnapshot> for String {
    fn from(vo: AppointmentServiceNameSnapshot) -> Self {
        vo.0
    }
}
