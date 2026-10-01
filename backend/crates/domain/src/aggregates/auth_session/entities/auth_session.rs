use chrono::NaiveDateTime;

use crate::aggregates::user::value_objects::UserId;

use super::super::{
    errors::AuthSessionError,
    value_objects::{
        AuthSessionId,
        AuthSessionRefreshTokenHash,
        AuthSessionUserAgent,
        AuthSessionIpAddress,
        AuthSessionExpiresAt,
        AuthSessionLastUsedAt,
        AuthSessionRevokedAt,
        AuthSessionCreatedAt
    }
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthSession {
    id: AuthSessionId,
    user_id: UserId,
    refresh_token_hash: AuthSessionRefreshTokenHash,
    user_agent: Option<AuthSessionUserAgent>,
    ip_address: Option<AuthSessionIpAddress>,
    expires_at: AuthSessionExpiresAt,
    last_used_at: Option<AuthSessionLastUsedAt>,
    revoked_at: Option<AuthSessionRevokedAt>,
    created_at: AuthSessionCreatedAt
}

// MARK: Constructors
impl AuthSession {
    pub fn create(
        user_id: UserId,
        refresh_token_hash: AuthSessionRefreshTokenHash,
        user_agent: Option<AuthSessionUserAgent>,
        ip_address: Option<AuthSessionIpAddress>,
        expires_at: AuthSessionExpiresAt,
        created_at: AuthSessionCreatedAt
    ) -> Result<Self, AuthSessionError> {
        Self::validate_expiration(created_at, expires_at)?;
        
        let id = AuthSessionId::generate();
        let last_used_at = None;
        let revoked_at = None;

        Ok(Self { id, user_id, refresh_token_hash, user_agent, ip_address, expires_at, last_used_at, revoked_at, created_at })
    }

    pub fn restore(
        id: AuthSessionId,
        user_id: UserId,
        refresh_token_hash: AuthSessionRefreshTokenHash,
        user_agent: Option<AuthSessionUserAgent>,
        ip_address: Option<AuthSessionIpAddress>,
        expires_at: AuthSessionExpiresAt,
        last_used_at: Option<AuthSessionLastUsedAt>,
        revoked_at: Option<AuthSessionRevokedAt>,
        created_at: AuthSessionCreatedAt
    ) -> Result<Self, AuthSessionError> {
        Self::validate_expiration(created_at, expires_at)?;
        Self::validate_last_used_at(created_at, last_used_at)?;
        Self::validate_revoked_at(created_at, revoked_at)?;
        Self::validate_last_used_before_revocation(last_used_at, revoked_at)?;
        
        Ok(Self { id, user_id, refresh_token_hash, user_agent, ip_address, expires_at, last_used_at, revoked_at, created_at })
    }
}

// MARK: Validation
impl AuthSession {
    fn validate_expiration(
        created_at: AuthSessionCreatedAt,
        expires_at: AuthSessionExpiresAt
    ) -> Result<(), AuthSessionError> {
        let created_at: NaiveDateTime = created_at.into();
        let expires_at: NaiveDateTime = expires_at.into();

        if expires_at <= created_at {
            return Err(AuthSessionError::InvalidExpirationTime);
        }

        Ok(())
    }

    fn validate_last_used_at(
        created_at: AuthSessionCreatedAt,
        last_used_at: Option<AuthSessionLastUsedAt>
    ) -> Result<(), AuthSessionError> {
        if let Some(last_used_at) = last_used_at {
            let created_at: NaiveDateTime = created_at.into();
            let last_used_at: NaiveDateTime = last_used_at.into();

            if last_used_at < created_at {
                return Err(AuthSessionError::InvalidLastUsedTime);
            }
        }

        Ok(())
    }

    fn validate_revoked_at(
        created_at: AuthSessionCreatedAt,
        revoked_at: Option<AuthSessionRevokedAt>
    ) -> Result<(), AuthSessionError> {
        if let Some(revoked_at) = revoked_at {
            let created_at: NaiveDateTime = created_at.into();
            let revoked_at: NaiveDateTime = revoked_at.into();

            if revoked_at < created_at {
                return Err(AuthSessionError::InvalidRevokedTime);
            }
        }

        Ok(())
    }

    fn validate_last_used_before_revocation(
        last_used_at: Option<AuthSessionLastUsedAt>,
        revoked_at: Option<AuthSessionRevokedAt>
    ) -> Result<(), AuthSessionError> {
        if let (Some(last_used_at), Some(revoked_at)) = (last_used_at, revoked_at) {
            let last_used_at: NaiveDateTime = last_used_at.into();
            let revoked_at: NaiveDateTime = revoked_at.into();

            if last_used_at > revoked_at {
                return Err(AuthSessionError::LastUsedAfterRevocation);
            }
        }

        Ok(())
    }
}

// MARK: Behavior
impl AuthSession {
    pub fn rotate_refresh_token(
        &mut self,
        refresh_token_hash: AuthSessionRefreshTokenHash,
        last_used_at: AuthSessionLastUsedAt
    ) -> Result<(), AuthSessionError> {
        if self.revoked_at.is_some() {
            return Err(AuthSessionError::AlreadyRevoked);
        }

        Self::validate_last_used_at(self.created_at, Some(last_used_at))?;

        self.refresh_token_hash = refresh_token_hash;
        self.last_used_at = Some(last_used_at);

        Ok(())
    }

    pub fn revoke(
        &mut self,
        revoked_at: AuthSessionRevokedAt
    ) -> Result<(), AuthSessionError> {
        if self.revoked_at.is_some() {
            return Err(AuthSessionError::AlreadyRevoked);
        }

        Self::validate_revoked_at(self.created_at, Some(revoked_at))?;
        Self::validate_last_used_before_revocation(self.last_used_at, Some(revoked_at))?;

        self.revoked_at = Some(revoked_at);

        Ok(())
    }
}

// MARK: Getters
impl AuthSession {
    pub fn id(&self) -> AuthSessionId {
        self.id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn refresh_token_hash(&self) -> &AuthSessionRefreshTokenHash {
        &self.refresh_token_hash
    }

    pub fn user_agent(&self) -> Option<&AuthSessionUserAgent> {
        self.user_agent.as_ref()
    }

    pub fn ip_address(&self) -> Option<&AuthSessionIpAddress> {
        self.ip_address.as_ref()
    }

    pub fn expires_at(&self) -> AuthSessionExpiresAt {
        self.expires_at
    }

    pub fn last_used_at(&self) -> Option<AuthSessionLastUsedAt> {
        self.last_used_at
    }

    pub fn revoked_at(&self) -> Option<AuthSessionRevokedAt> {
        self.revoked_at
    }
    
    pub fn created_at(&self) -> AuthSessionCreatedAt {
        self.created_at
    }

    pub fn is_revoked(&self) -> bool {
        self.revoked_at.is_some()
    }
}
