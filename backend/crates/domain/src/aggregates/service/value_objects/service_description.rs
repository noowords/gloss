use super::super::errors::ServiceError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceDescription(String);

// MARK: Conversions
impl TryFrom<String> for ServiceDescription {
    type Error = ServiceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(ServiceError::DescriptionEmpty);
        }

        if value.chars().count() > 1024 {
            return Err(ServiceError::DescriptionTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<ServiceDescription> for String {
    fn from(vo: ServiceDescription) -> Self {
        vo.0
    }
}
