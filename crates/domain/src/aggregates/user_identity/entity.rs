use crate::aggregates::user::value_objects::{ UserId };

use super::value_objects::{ UserIdentityId, UserIdentityProviderType, UserIdentityProviderKey, UserIdentityProviderData };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserIdentity {
    id: UserIdentityId,
    user_id: UserId,
    provider_type: UserIdentityProviderType,
    provider_key: UserIdentityProviderKey,
    provider_data: Option<UserIdentityProviderData>
}

impl UserIdentity {
    pub fn create(
        user_id: UserId,
        provider_type: UserIdentityProviderType,
        provider_key: UserIdentityProviderKey,
        provider_data: Option<UserIdentityProviderData>
    ) -> Self {
        Self {
            id: UserIdentityId::generate(),
            user_id,
            provider_type,
            provider_key,
            provider_data
        }
    }

    pub fn restore(
        id: UserIdentityId,
        user_id: UserId,
        provider_type: UserIdentityProviderType,
        provider_key: UserIdentityProviderKey,
        provider_data: Option<UserIdentityProviderData>
    ) -> Self {
        Self {
            id,
            user_id,
            provider_type,
            provider_key,
            provider_data
        }
    }

    pub fn id(&self) -> UserIdentityId {
        self.id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn provider_type(&self) -> UserIdentityProviderType {
        self.provider_type
    }

    pub fn provider_key(&self) -> UserIdentityProviderKey {
        self.provider_key.clone()
    }

    pub fn provider_data(&self) -> Option<UserIdentityProviderData> {
        self.provider_data.clone()
    }
}
