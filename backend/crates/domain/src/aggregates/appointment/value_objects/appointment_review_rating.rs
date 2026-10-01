use super::super::errors::AppointmentReviewError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppointmentReviewRating(u8);

// MARK: Conversions
impl TryFrom<u8> for AppointmentReviewRating {
    type Error = AppointmentReviewError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=5).contains(&value) {
            return Err(AppointmentReviewError::RatingOutOfRange);
        }

        Ok(Self(value))
    }
}

impl From<AppointmentReviewRating> for u8 {
    fn from(vo: AppointmentReviewRating) -> Self {
        vo.0
    }
}
