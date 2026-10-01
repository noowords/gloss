use super::super::{
    errors::UserProviderError,
    value_objects::{
        UserId,
        UserProviderId,
        UserProviderType,
        UserProviderSubject,
        UserProviderVerifiedAt
    }
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserProvider {
    id: UserProviderId,
    user_id: UserId,
    r#type: UserProviderType,
    subject: UserProviderSubject,
    verified_at: Option<UserProviderVerifiedAt>
}

// MARK: Constructors
impl UserProvider {
    pub fn create(
        user_id: UserId,
        r#type: UserProviderType,
        subject: UserProviderSubject
    ) -> Self {
        let id = UserProviderId::generate();
        let verified_at = None;

        Self { id, user_id, r#type, subject, verified_at }
    }

    pub fn restore(
        id: UserProviderId,
        user_id: UserId,
        r#type: UserProviderType,
        subject: UserProviderSubject,
        verified_at: Option<UserProviderVerifiedAt>
    ) -> Self {
        Self { id, user_id, r#type, subject, verified_at }
    }
}

// MARK: Behavior
impl UserProvider {
    pub fn verify(
        &mut self,
        verified_at: UserProviderVerifiedAt
    ) -> Result<(), UserProviderError> {
        if self.verified_at.is_some() {
            return Err(UserProviderError::AlreadyVerified);
        }
    
        self.verified_at = Some(verified_at);
    
        Ok(())
    }
}

// MARK: Getters
impl UserProvider {
    pub fn id(&self) -> UserProviderId {
        self.id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn r#type(&self) -> UserProviderType {
        self.r#type
    }

    pub fn subject(&self) -> &UserProviderSubject {
        &self.subject
    }

    pub fn verified_at(&self) -> Option<UserProviderVerifiedAt> {
        self.verified_at
    }

    pub fn is_verified(&self) -> bool {
        self.verified_at.is_some()
    }
}
