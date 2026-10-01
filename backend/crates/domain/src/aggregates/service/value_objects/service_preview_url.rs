use super::super::errors::ServiceError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServicePreviewUrl(String);

// MARK: Conversions
impl TryFrom<String> for ServicePreviewUrl {
    type Error = ServiceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(ServiceError::PreviewUrlEmpty);
        }

        if value.chars().count() > 2048 {
            return Err(ServiceError::PreviewUrlTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<ServicePreviewUrl> for String {
    fn from(vo: ServicePreviewUrl) -> Self {
        vo.0
    }
}
