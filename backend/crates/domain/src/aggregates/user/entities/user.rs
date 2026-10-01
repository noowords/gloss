use super::super::value_objects::{ UserId, UserStatus };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    id: UserId,
    status: UserStatus
}

// MARK: Constructors
impl User {
    pub fn create() -> Self {
        let id = UserId::generate();
        let status = UserStatus::Active;

        Self { id, status }
    }

    pub fn restore(id: UserId, status: UserStatus) -> Self {
        Self { id, status }
    }
}

// MARK: Behavior
impl User {
    pub fn block(&mut self) {
        self.status = UserStatus::Blocked;
    }

    pub fn unblock(&mut self) {
        self.status = UserStatus::Active;
    }
}

// MARK: Getters
impl User {
    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn status(&self) -> UserStatus {
        self.status
    }

    pub fn is_active(&self) -> bool {
        self.status == UserStatus::Active
    }
    
    pub fn is_blocked(&self) -> bool {
        self.status == UserStatus::Blocked
    }
}
