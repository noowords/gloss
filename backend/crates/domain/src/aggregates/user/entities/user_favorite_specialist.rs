use crate::aggregates::{
    user::value_objects::UserId,
    specialist::value_objects::SpecialistId
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserFavoriteSpecialist {
    user_id: UserId,
    specialist_id: SpecialistId
}

// MARK: Constructors
impl UserFavoriteSpecialist {
    pub fn create(user_id: UserId, specialist_id: SpecialistId) -> Self {
        Self { user_id, specialist_id }
    }

    pub fn restore(user_id: UserId, specialist_id: SpecialistId) -> Self {
        Self { user_id, specialist_id }
    }
}

// MARK: Getters
impl UserFavoriteSpecialist {
    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn specialist_id(&self) -> SpecialistId {
        self.specialist_id
    }
}
