use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OtpChallengeId(Uuid);

impl OtpChallengeId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<OtpChallengeId> for Uuid {
    fn from(id: OtpChallengeId) -> Self {
        id.0
    }
}

impl From<OtpChallengeId> for String {
    fn from(id: OtpChallengeId) -> Self {
        id.0.to_string()
    }
}

impl From<OtpChallengeId> for [u8; 16] {
    fn from(id: OtpChallengeId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for OtpChallengeId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for OtpChallengeId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(OtpChallengeId)
            .map_err(|_| anyhow::anyhow!("Invalid OtpChallengeId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for OtpChallengeId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(OtpChallengeId)
            .map_err(|_| anyhow::anyhow!("Invalid OtpChallengeId: {}", String::from_utf8_lossy(bytes)))
    }
}
