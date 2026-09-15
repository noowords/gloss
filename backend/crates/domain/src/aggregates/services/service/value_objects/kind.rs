#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ServiceKind {
    Primary,
    Addon
}

impl From<ServiceKind> for String {
    fn from(value: ServiceKind) -> Self {
        match value {
            ServiceKind::Primary => "primary",
            ServiceKind::Addon => "addon"
        }.to_string()
    }
}

impl TryFrom<String> for ServiceKind {
    type Error = anyhow::Error;

    fn try_from(str: String) -> Result<Self, Self::Error> {
        Self::try_from(str.as_str())
    }
}

impl TryFrom<&str> for ServiceKind {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str {
            "primary" => Ok(ServiceKind::Primary),
            "addon" => Ok(ServiceKind::Addon),
            _ => anyhow::bail!("Invalid ServiceKind: {}", str)
        }
    }
}
