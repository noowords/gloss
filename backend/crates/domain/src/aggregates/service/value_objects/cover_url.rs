#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceCoverUrl(String);

impl From<ServiceCoverUrl> for String {
    fn from(cover_url: ServiceCoverUrl) -> Self {
        cover_url.0
    }
}

impl From<String> for ServiceCoverUrl {
    fn from(str: String) -> Self {
        Self(str)
    }
}

impl From<&str> for ServiceCoverUrl {
    fn from(str: &str) -> Self {
        Self(str.to_string())
    }
}
