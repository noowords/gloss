use uuid::{ Uuid };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ReviewId(Uuid);

impl ReviewId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<ReviewId> for Uuid {
    fn from(id: ReviewId) -> Self {
        id.0
    }
}

impl From<ReviewId> for String {
    fn from(id: ReviewId) -> Self {
        id.0.to_string()
    }
}

impl From<ReviewId> for [u8; 16] {
    fn from(id: ReviewId) -> Self {
        id.0.into_bytes()
    }
}

impl From<Uuid> for ReviewId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl TryFrom<&str> for ReviewId {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(str)
            .map(ReviewId)
            .map_err(|_| anyhow::anyhow!("Invalid ReviewId: {}", str))
    }
}

impl TryFrom<&[u8; 16]> for ReviewId {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Uuid::from_slice(bytes)
            .map(ReviewId)
            .map_err(|_| anyhow::anyhow!("Invalid ReviewId: {}", String::from_utf8_lossy(bytes)))
    }
}
