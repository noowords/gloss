#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtpCode(String);

impl From<OtpCode> for String {
    fn from(code: OtpCode) -> Self {
        code.0
    }
}

impl From<String> for OtpCode {
    fn from(str: String) -> Self {
        Self(str)
    }
}

impl From<&str> for OtpCode {
    fn from(str: &str) -> Self {
        Self(str.to_string())
    }
}
