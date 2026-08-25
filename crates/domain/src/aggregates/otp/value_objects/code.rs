use rand::{ RngExt };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtpCode(String);

impl OtpCode {
    pub fn generate_random_numeric(length: usize) -> Self {
        let mut rng = rand::rng();
        
        Self((0..length).map(|_| rng.random_range(0..10).to_string()).collect())
    }
}

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
