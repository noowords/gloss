#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ServiceCategory {
    Manicure,
    Pedicure
}

impl From<ServiceCategory> for String {
    fn from(category: ServiceCategory) -> Self {
        match category {
            ServiceCategory::Manicure => "manicure",
            ServiceCategory::Pedicure => "pedicure"
        }.to_string()
    }
}

impl TryFrom<String> for ServiceCategory {
    type Error = anyhow::Error;
    
    fn try_from(str: String) -> Result<Self, Self::Error> {
        match str.as_str() {
            "manicure" => Some(ServiceCategory::Manicure),
            "pedicure" => Some(ServiceCategory::Pedicure),
            _ => None
        }.ok_or_else(|| anyhow::anyhow!("Invalid ServiceCategory: {}", str.to_string()))
    }
}

impl TryFrom<&str> for ServiceCategory {
    type Error = anyhow::Error;
    
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str {
            "manicure" => Some(ServiceCategory::Manicure),
            "pedicure" => Some(ServiceCategory::Pedicure),
            _ => None
        }.ok_or_else(|| anyhow::anyhow!("Invalid ServiceCategory: {}", str.to_string()))
    }
}
