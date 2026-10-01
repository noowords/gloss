use super::super::value_objects::{
    AppointmentId,
    AppointmentReviewId,
    AppointmentReviewRating,
    AppointmentReviewComment,
    AppointmentReviewStatus,
    AppointmentReviewCreatedAt
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppointmentReview {
    id: AppointmentReviewId,
    appointment_id: AppointmentId,
    rating: AppointmentReviewRating,
    comment: Option<AppointmentReviewComment>,
    status: AppointmentReviewStatus,
    created_at: AppointmentReviewCreatedAt
}

// MARK: Constructors
impl AppointmentReview {
    pub fn create(
        appointment_id: AppointmentId,
        rating: AppointmentReviewRating,
        comment: Option<AppointmentReviewComment>,
        created_at: AppointmentReviewCreatedAt
    ) -> Self {
        let id = AppointmentReviewId::generate();
        let status = AppointmentReviewStatus::Published;

        Self { id, appointment_id, rating, comment, status, created_at }
    }

    pub fn restore(
        id: AppointmentReviewId,
        appointment_id: AppointmentId,
        rating: AppointmentReviewRating,
        comment: Option<AppointmentReviewComment>,
        status: AppointmentReviewStatus,
        created_at: AppointmentReviewCreatedAt
    ) -> Self {
        Self { id, appointment_id, rating, comment, status, created_at }
    }
}

// MARK: Behavior
impl AppointmentReview {
    pub fn change_rating(&mut self, rating: AppointmentReviewRating) {
        self.rating = rating;
    }

    pub fn change_comment(&mut self, comment: Option<AppointmentReviewComment>) {
        self.comment = comment;
    }

    pub fn publish(&mut self) {
        self.status = AppointmentReviewStatus::Published;
    }

    pub fn hide(&mut self) {
        self.status = AppointmentReviewStatus::Hidden;
    }
}

// MARK: Getters
impl AppointmentReview {
    pub fn id(&self) -> AppointmentReviewId {
        self.id
    }

    pub fn appointment_id(&self) -> AppointmentId {
        self.appointment_id
    }

    pub fn rating(&self) -> AppointmentReviewRating {
        self.rating
    }

    pub fn comment(&self) -> Option<&AppointmentReviewComment> {
        self.comment.as_ref()
    }

    pub fn status(&self) -> AppointmentReviewStatus {
        self.status
    }
    
    pub fn created_at(&self) -> AppointmentReviewCreatedAt {
        self.created_at
    }

    pub fn is_published(&self) -> bool {
        self.status == AppointmentReviewStatus::Published
    }
}
