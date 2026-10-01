use chrono::NaiveDateTime;

use super::super::{
    errors::OtpChallengeError,
    value_objects::{
        OtpChallengeId,
        OtpChallengeSubject,
        OtpChallengePurpose,
        OtpChallengeCodeHash,
        OtpChallengeAttempts,
        OtpChallengeExpiresAt,
        OtpChallengeVerifiedAt,
        OtpChallengeConsumedAt,
        OtpChallengeCreatedAt
    }
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtpChallenge {
    id: OtpChallengeId,
    subject: OtpChallengeSubject,
    purpose: OtpChallengePurpose,
    code_hash: OtpChallengeCodeHash,
    attempts: OtpChallengeAttempts,
    expires_at: OtpChallengeExpiresAt,
    verified_at: Option<OtpChallengeVerifiedAt>,
    consumed_at: Option<OtpChallengeConsumedAt>,
    created_at: OtpChallengeCreatedAt
}

// MARK: Constructors
impl OtpChallenge {
    pub fn create(
        subject: OtpChallengeSubject,
        purpose: OtpChallengePurpose,
        code_hash: OtpChallengeCodeHash,
        expires_at: OtpChallengeExpiresAt,
        created_at: OtpChallengeCreatedAt
    ) -> Result<Self, OtpChallengeError> {
        Self::validate_expiration(created_at, expires_at)?;
        
        let id = OtpChallengeId::generate();
        let attempts = OtpChallengeAttempts::from(0);
        let verified_at = None;
        let consumed_at = None;
        
        Ok(Self { id, subject, purpose, code_hash, attempts, expires_at, verified_at, consumed_at, created_at })
    }

    pub fn restore(
        id: OtpChallengeId,
        subject: OtpChallengeSubject,
        purpose: OtpChallengePurpose,
        code_hash: OtpChallengeCodeHash,
        attempts: OtpChallengeAttempts,
        expires_at: OtpChallengeExpiresAt,
        verified_at: Option<OtpChallengeVerifiedAt>,
        consumed_at: Option<OtpChallengeConsumedAt>,
        created_at: OtpChallengeCreatedAt
    ) -> Result<Self, OtpChallengeError> {
        Self::validate_expiration(created_at, expires_at)?;
        Self::validate_verification(created_at, expires_at, verified_at)?;
        Self::validate_consumption(verified_at, consumed_at)?;
        
        Ok(Self { id, subject, purpose, code_hash, attempts, expires_at, verified_at, consumed_at, created_at })
    }
}

// MARK: Validation
impl OtpChallenge {
    fn validate_expiration(
        created_at: OtpChallengeCreatedAt,
        expires_at: OtpChallengeExpiresAt
    ) -> Result<(), OtpChallengeError> {
        let created_at: NaiveDateTime = created_at.into();
        let expires_at: NaiveDateTime = expires_at.into();

        if expires_at <= created_at {
            return Err(OtpChallengeError::InvalidExpirationTime);
        }

        Ok(())
    }

    fn validate_verification(
        created_at: OtpChallengeCreatedAt,
        expires_at: OtpChallengeExpiresAt,
        verified_at: Option<OtpChallengeVerifiedAt>
    ) -> Result<(), OtpChallengeError> {
        let Some(verified_at) = verified_at else {
            return Ok(());
        };

        let created_at: NaiveDateTime = created_at.into();
        let expires_at: NaiveDateTime = expires_at.into();
        let verified_at: NaiveDateTime = verified_at.into();

        if verified_at < created_at {
            return Err(OtpChallengeError::InvalidVerificationTime);
        }

        if verified_at > expires_at {
            return Err(OtpChallengeError::VerificationAfterExpiration);
        }

        Ok(())
    }

    fn validate_consumption(
        verified_at: Option<OtpChallengeVerifiedAt>,
        consumed_at: Option<OtpChallengeConsumedAt>
    ) -> Result<(), OtpChallengeError> {
        let Some(consumed_at) = consumed_at else {
            return Ok(());
        };

        let Some(verified_at) = verified_at else {
            return Err(OtpChallengeError::ConsumedWithoutVerification);
        };

        let verified_at: NaiveDateTime = verified_at.into();
        let consumed_at: NaiveDateTime = consumed_at.into();

        if consumed_at < verified_at {
            return Err(OtpChallengeError::InvalidConsumptionTime);
        }

        Ok(())
    }
}

// MARK: Behavior
impl OtpChallenge {
    pub fn increment_attempts(&mut self) {
        self.attempts = OtpChallengeAttempts::from(
            u16::from(self.attempts) + 1
        );
    }

    pub fn verify(
        &mut self,
        verified_at: OtpChallengeVerifiedAt
    ) -> Result<(), OtpChallengeError> {
        if self.verified_at.is_some() {
            return Err(OtpChallengeError::AlreadyVerified);
        }

        Self::validate_verification(self.created_at, self.expires_at, Some(verified_at))?;

        self.verified_at = Some(verified_at);

        Ok(())
    }

    pub fn consume(
        &mut self,
        consumed_at: OtpChallengeConsumedAt,
    ) -> Result<(), OtpChallengeError> {
        if self.consumed_at.is_some() {
            return Err(OtpChallengeError::AlreadyConsumed);
        }

        Self::validate_consumption(self.verified_at, Some(consumed_at))?;

        self.consumed_at = Some(consumed_at);

        Ok(())
    }
}

// MARK: Getters
impl OtpChallenge {
    pub fn id(&self) -> OtpChallengeId {
        self.id
    }

    pub fn subject(&self) -> &OtpChallengeSubject {
        &self.subject
    }

    pub fn purpose(&self) -> OtpChallengePurpose {
        self.purpose
    }

    pub fn code_hash(&self) -> &OtpChallengeCodeHash {
        &self.code_hash
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
    
    pub fn created_at(&self) -> OtpChallengeCreatedAt {
        self.created_at
    }

    pub fn is_verified(&self) -> bool {
        self.verified_at.is_some()
    }
    
    pub fn is_consumed(&self) -> bool {
        self.consumed_at.is_some()
    }
}
