use crate::aggregates::users::user::value_objects::{ UserId };

use super::value_objects::{ UserRoleName };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserRole {
    user_id: UserId,
    role: UserRoleName
}

impl UserRole {
    pub fn create(
        user_id: UserId,
        role: UserRoleName
    ) -> Result<Self, anyhow::Error> {
        Self::restore(
            user_id,
            role
        )
    }

    pub fn restore(
        user_id: UserId,
        role: UserRoleName
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            user_id,
            role
        })
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn role(&self) -> UserRoleName {
        self.role.clone()
    }
}
