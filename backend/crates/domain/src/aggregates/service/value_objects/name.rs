#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceName(String);

impl From<ServiceName> for String {
    fn from(name: ServiceName) -> Self {
        name.0
    }
}

impl From<String> for ServiceName {
    fn from(str: String) -> Self {
        Self(str)
    }
}

impl From<&str> for ServiceName {
    fn from(str: &str) -> Self {
        Self(str.to_string())
    }
}
