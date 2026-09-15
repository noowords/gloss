use crate::aggregates::users::user::value_objects::{ UserId };

use super::value_objects::{ AuthSessionId, AuthSessionRefreshTokenHash, AuthSessionUserAgent, AuthSessionIpAddress, AuthSessionExpiresAt, AuthSessionLastUsedAt, AuthSessionRevokedAt };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthSession {
    id: AuthSessionId,
    user_id: UserId,
    refresh_token_hash: AuthSessionRefreshTokenHash,
    user_agent: Option<AuthSessionUserAgent>,
    ip_address: Option<AuthSessionIpAddress>,
    expires_at: AuthSessionExpiresAt,
    last_used_at: Option<AuthSessionLastUsedAt>,
    revoked_at: Option<AuthSessionRevokedAt>
}

impl AuthSession {
    pub fn create(
        user_id: UserId,
        refresh_token_hash: AuthSessionRefreshTokenHash,
        user_agent: Option<AuthSessionUserAgent>,
        ip_address: Option<AuthSessionIpAddress>,
        expires_at: AuthSessionExpiresAt,
        last_used_at: Option<AuthSessionLastUsedAt>,
        revoked_at: Option<AuthSessionRevokedAt>
    ) -> Result<Self, anyhow::Error> {
        let id = AuthSessionId::generate();
        Self::restore(
            id,
            user_id,
            refresh_token_hash,
            user_agent,
            ip_address,
            expires_at,
            last_used_at,
            revoked_at
        )
    }

    pub fn restore(
        id: AuthSessionId,
        user_id: UserId,
        refresh_token_hash: AuthSessionRefreshTokenHash,
        user_agent: Option<AuthSessionUserAgent>,
        ip_address: Option<AuthSessionIpAddress>,
        expires_at: AuthSessionExpiresAt,
        last_used_at: Option<AuthSessionLastUsedAt>,
        revoked_at: Option<AuthSessionRevokedAt>
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            id,
            user_id,
            refresh_token_hash,
            user_agent,
            ip_address,
            expires_at,
            last_used_at,
            revoked_at
        })
    }

    pub fn id(&self) -> AuthSessionId {
        self.id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn refresh_token_hash(&self) -> AuthSessionRefreshTokenHash {
        self.refresh_token_hash.clone()
    }

    pub fn user_agent(&self) -> Option<AuthSessionUserAgent> {
        self.user_agent.clone()
    }

    pub fn ip_address(&self) -> Option<AuthSessionIpAddress> {
        self.ip_address.clone()
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
}
