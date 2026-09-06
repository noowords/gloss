use crate::aggregates::user::value_objects::{ UserId };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Specialist {
    user_id: UserId
}

impl Specialist {
    pub fn create(user_id: UserId) -> Self {
        Self { user_id }
    }
    
    pub fn restore(user_id: UserId) -> Self {
        Self { user_id }
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }
}
