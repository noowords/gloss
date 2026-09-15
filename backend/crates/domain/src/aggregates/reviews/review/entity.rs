use crate::aggregates::appointments::appointment::value_objects::{ AppointmentId };

use super::value_objects::{ ReviewId, ReviewRating, ReviewComment, ReviewStatus };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Review {
    id: ReviewId,
    appointment_id: AppointmentId,
    rating: ReviewRating,
    comment: Option<ReviewComment>,
    status: ReviewStatus
}

impl Review {
    pub fn create(
        appointment_id: AppointmentId,
        rating: ReviewRating,
        comment: Option<ReviewComment>
    ) -> Result<Self, anyhow::Error> {
        let status = ReviewStatus::try_from("published")?;
        let id = ReviewId::generate();
        Self::restore(
            id,
            appointment_id,
            rating,
            comment,
            status
        )
    }

    pub fn restore(
        id: ReviewId,
        appointment_id: AppointmentId,
        rating: ReviewRating,
        comment: Option<ReviewComment>,
        status: ReviewStatus
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            id,
            appointment_id,
            rating,
            comment,
            status
        })
    }

    pub fn id(&self) -> ReviewId {
        self.id
    }

    pub fn appointment_id(&self) -> AppointmentId {
        self.appointment_id
    }

    pub fn rating(&self) -> ReviewRating {
        self.rating
    }

    pub fn comment(&self) -> Option<ReviewComment> {
        self.comment.clone()
    }

    pub fn status(&self) -> ReviewStatus {
        self.status.clone()
    }
}
