use super::super::value_objects::{ UserId, UserRoleName };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserRole {
    user_id: UserId,
    name: UserRoleName
}

// MARK: Constructors
impl UserRole {
    pub fn create(user_id: UserId, name: UserRoleName) -> Self {
        Self { user_id, name }
    }

    pub fn restore(user_id: UserId, name: UserRoleName) -> Self {
        Self { user_id, name }
    }
}

// MARK: Getters
impl UserRole {
    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn name(&self) -> UserRoleName {
        self.name
    }
}
