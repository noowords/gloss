use crate::aggregates::users::user::value_objects::{ UserId };
use crate::aggregates::salons::salon::value_objects::{ SalonId };

use super::value_objects::{ SpecialistId, SpecialistBio, SpecialistExperienceStartedAt, SpecialistStatus };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Specialist {
    id: SpecialistId,
    user_id: UserId,
    salon_id: SalonId,
    bio: Option<SpecialistBio>,
    experience_started_at: Option<SpecialistExperienceStartedAt>,
    status: SpecialistStatus
}

impl Specialist {
    pub fn create(
        user_id: UserId,
        salon_id: SalonId,
        bio: Option<SpecialistBio>,
        experience_started_at: Option<SpecialistExperienceStartedAt>
    ) -> Result<Self, anyhow::Error> {
        let status = SpecialistStatus::try_from("active")?;
        let id = SpecialistId::generate();
        Self::restore(
            id,
            user_id,
            salon_id,
            bio,
            experience_started_at,
            status
        )
    }

    pub fn restore(
        id: SpecialistId,
        user_id: UserId,
        salon_id: SalonId,
        bio: Option<SpecialistBio>,
        experience_started_at: Option<SpecialistExperienceStartedAt>,
        status: SpecialistStatus
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            id,
            user_id,
            salon_id,
            bio,
            experience_started_at,
            status
        })
    }

    pub fn id(&self) -> SpecialistId {
        self.id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn salon_id(&self) -> SalonId {
        self.salon_id
    }

    pub fn bio(&self) -> Option<SpecialistBio> {
        self.bio.clone()
    }

    pub fn experience_started_at(&self) -> Option<SpecialistExperienceStartedAt> {
        self.experience_started_at
    }

    pub fn status(&self) -> SpecialistStatus {
        self.status.clone()
    }
}
