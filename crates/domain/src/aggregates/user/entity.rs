use super::value_objects::{ UserId, UserRole };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    id: UserId,
    role: UserRole
}

impl User {
    pub fn create() -> Self {
        let id = UserId::generate();
        
        Self {
            id,
            role: UserRole::Client
        }
    }
    
    pub fn restore(
        id: UserId,
        role: UserRole
    ) -> Self {
        Self { id, role }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn role(&self) -> UserRole {
        self.role
    }
}
