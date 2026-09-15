use crate::aggregates::users::user::value_objects::{ UserId };

use super::value_objects::{ UserProviderId, UserProviderProvider, UserProviderSubject, UserProviderVerifiedAt };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserProvider {
    id: UserProviderId,
    user_id: UserId,
    provider: UserProviderProvider,
    subject: UserProviderSubject,
    verified_at: Option<UserProviderVerifiedAt>
}

impl UserProvider {
    pub fn create(
        user_id: UserId,
        provider: UserProviderProvider,
        subject: UserProviderSubject,
        verified_at: Option<UserProviderVerifiedAt>
    ) -> Result<Self, anyhow::Error> {
        let id = UserProviderId::generate();
        Self::restore(
            id,
            user_id,
            provider,
            subject,
            verified_at
        )
    }

    pub fn restore(
        id: UserProviderId,
        user_id: UserId,
        provider: UserProviderProvider,
        subject: UserProviderSubject,
        verified_at: Option<UserProviderVerifiedAt>
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            id,
            user_id,
            provider,
            subject,
            verified_at
        })
    }

    pub fn id(&self) -> UserProviderId {
        self.id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn provider(&self) -> UserProviderProvider {
        self.provider.clone()
    }

    pub fn subject(&self) -> UserProviderSubject {
        self.subject.clone()
    }

    pub fn verified_at(&self) -> Option<UserProviderVerifiedAt> {
        self.verified_at
    }
}
