use super::value_objects::{ OtpChallengeId, OtpChallengeProvider, OtpChallengeSubject, OtpChallengePurpose, OtpChallengeCodeHash, OtpChallengeAttempts, OtpChallengeExpiresAt, OtpChallengeVerifiedAt, OtpChallengeConsumedAt };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtpChallenge {
    id: OtpChallengeId,
    provider: OtpChallengeProvider,
    subject: OtpChallengeSubject,
    purpose: OtpChallengePurpose,
    code_hash: OtpChallengeCodeHash,
    attempts: OtpChallengeAttempts,
    expires_at: OtpChallengeExpiresAt,
    verified_at: Option<OtpChallengeVerifiedAt>,
    consumed_at: Option<OtpChallengeConsumedAt>
}

impl OtpChallenge {
    pub fn create(
        provider: OtpChallengeProvider,
        subject: OtpChallengeSubject,
        purpose: OtpChallengePurpose,
        code_hash: OtpChallengeCodeHash,
        expires_at: OtpChallengeExpiresAt,
        verified_at: Option<OtpChallengeVerifiedAt>,
        consumed_at: Option<OtpChallengeConsumedAt>
    ) -> Result<Self, anyhow::Error> {
        let attempts = OtpChallengeAttempts::try_from(0u16)?;
        let id = OtpChallengeId::generate();
        Self::restore(
            id,
            provider,
            subject,
            purpose,
            code_hash,
            attempts,
            expires_at,
            verified_at,
            consumed_at
        )
    }

    pub fn restore(
        id: OtpChallengeId,
        provider: OtpChallengeProvider,
        subject: OtpChallengeSubject,
        purpose: OtpChallengePurpose,
        code_hash: OtpChallengeCodeHash,
        attempts: OtpChallengeAttempts,
        expires_at: OtpChallengeExpiresAt,
        verified_at: Option<OtpChallengeVerifiedAt>,
        consumed_at: Option<OtpChallengeConsumedAt>
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            id,
            provider,
            subject,
            purpose,
            code_hash,
            attempts,
            expires_at,
            verified_at,
            consumed_at
        })
    }

    pub fn id(&self) -> OtpChallengeId {
        self.id
    }

    pub fn provider(&self) -> OtpChallengeProvider {
        self.provider.clone()
    }

    pub fn subject(&self) -> OtpChallengeSubject {
        self.subject.clone()
    }

    pub fn purpose(&self) -> OtpChallengePurpose {
        self.purpose.clone()
    }

    pub fn code_hash(&self) -> OtpChallengeCodeHash {
        self.code_hash.clone()
    }

    pub fn attempts(&self) -> OtpChallengeAttempts {
        self.attempts
    }

    pub fn expires_at(&self) -> OtpChallengeExpiresAt {
        self.expires_at
    }

    pub fn verified_at(&self) -> Option<OtpChallengeVerifiedAt> {
        self.verified_at
    }

    pub fn consumed_at(&self) -> Option<OtpChallengeConsumedAt> {
        self.consumed_at
    }
}
