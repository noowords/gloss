use crate::aggregates::users::user::value_objects::{ UserId };
use crate::aggregates::specialists::specialist::value_objects::{ SpecialistId };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FavoriteSpecialist {
    user_id: UserId,
    specialist_id: SpecialistId
}

impl FavoriteSpecialist {
    pub fn create(
        user_id: UserId,
        specialist_id: SpecialistId
    ) -> Result<Self, anyhow::Error> {
        Self::restore(
            user_id,
            specialist_id
        )
    }

    pub fn restore(
        user_id: UserId,
        specialist_id: SpecialistId
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            user_id,
            specialist_id
        })
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn specialist_id(&self) -> SpecialistId {
        self.specialist_id
    }
}
