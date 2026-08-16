#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceDescription(String);

impl From<ServiceDescription> for String {
    fn from(description: ServiceDescription) -> Self {
        description.0
    }
}

impl From<String> for ServiceDescription {
    fn from(str: String) -> Self {
        Self(str)
    }
}

impl From<&str> for ServiceDescription {
    fn from(str: &str) -> Self {
        Self(str.to_string())
    }
}
