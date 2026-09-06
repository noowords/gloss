use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OtpId(Uuid);

impl OtpId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<OtpId> for Uuid {
    fn from(id: OtpId) -> Self {
        id.0
    }
}

impl From<OtpId> for String {
    fn from(id: OtpId) -> Self {
        id.0.to_string()
    }
}

impl From<OtpId> for [u8; 16] {
    fn from(id: OtpId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for OtpId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for OtpId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(OtpId)
            .map_err(|_| anyhow::anyhow!("Invalid UserId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for OtpId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(OtpId)
            .map_err(|_| anyhow::anyhow!("Invalid OtpId: {}", String::from_utf8_lossy(bytes)))
    }
}
