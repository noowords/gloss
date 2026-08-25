#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtpProviderKey(String);

impl From<OtpProviderKey> for String {
    fn from(provider_key: OtpProviderKey) -> Self {
        provider_key.0
    }
}

impl From<String> for OtpProviderKey {
    fn from(str: String) -> Self {
        Self(str)
    }
}

impl From<&str> for OtpProviderKey {
    fn from(str: &str) -> Self {
        Self(str.to_string())
    }
}
