use super::super::errors::ServiceError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceName(String);

// MARK: Conversions
impl TryFrom<String> for ServiceName {
    type Error = ServiceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(ServiceError::NameEmpty);
        }

        if value.chars().count() > 128 {
            return Err(ServiceError::NameTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<ServiceName> for String {
    fn from(vo: ServiceName) -> Self {
        vo.0
    }
}
