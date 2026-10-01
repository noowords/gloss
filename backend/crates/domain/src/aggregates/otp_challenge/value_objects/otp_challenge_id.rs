use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct OtpChallengeId(Uuid);

// MARK: Constructors
impl OtpChallengeId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

// MARK: Conversions
impl From<Uuid> for OtpChallengeId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<OtpChallengeId> for Uuid {
    fn from(vo: OtpChallengeId) -> Self {
        vo.0
    }
}
