use crate::aggregates::{
    user::value_objects::UserId,
    salon::value_objects::SalonId
};

use super::super::value_objects::{
    SpecialistId,
    SpecialistBio,
    SpecialistExperienceStartedAt,
    SpecialistStatus
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Specialist {
    id: SpecialistId,
    user_id: UserId,
    salon_id: SalonId,
    bio: Option<SpecialistBio>,
    experience_started_at: Option<SpecialistExperienceStartedAt>,
    status: SpecialistStatus
}

// MARK: Constructors
impl Specialist {
    pub fn create(
        user_id: UserId,
        salon_id: SalonId,
        bio: Option<SpecialistBio>,
        experience_started_at: Option<SpecialistExperienceStartedAt>
    ) -> Self {
        let id = SpecialistId::generate();
        let status = SpecialistStatus::Active;

        Self { id, user_id, salon_id, bio, experience_started_at, status }
    }

    pub fn restore(
        id: SpecialistId,
        user_id: UserId,
        salon_id: SalonId,
        bio: Option<SpecialistBio>,
        experience_started_at: Option<SpecialistExperienceStartedAt>,
        status: SpecialistStatus
    ) -> Self {
        Self { id, user_id, salon_id, bio, experience_started_at, status }
    }
}

// MARK: Behavior
impl Specialist {
    pub fn change_bio(&mut self, bio: Option<SpecialistBio>) {
        self.bio = bio;
    }

    pub fn change_experience_started_at(
        &mut self,
        experience_started_at: Option<SpecialistExperienceStartedAt>,
    ) {
        self.experience_started_at = experience_started_at;
    }

    pub fn activate(&mut self) {
        self.status = SpecialistStatus::Active;
    }

    pub fn deactivate(&mut self) {
        self.status = SpecialistStatus::Inactive;
    }
}

// MARK: Getters
impl Specialist {
    pub fn id(&self) -> SpecialistId {
        self.id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn salon_id(&self) -> SalonId {
        self.salon_id
    }

    pub fn bio(&self) -> Option<&SpecialistBio> {
        self.bio.as_ref()
    }

    pub fn experience_started_at(&self) -> Option<SpecialistExperienceStartedAt> {
        self.experience_started_at
    }

    pub fn status(&self) -> SpecialistStatus {
        self.status
    }

    pub fn is_active(&self) -> bool {
        self.status == SpecialistStatus::Active
    }
}
