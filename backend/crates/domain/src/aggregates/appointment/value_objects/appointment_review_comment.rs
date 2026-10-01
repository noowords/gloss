use super::super::errors::AppointmentReviewError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppointmentReviewComment(String);

// MARK: Conversions
impl TryFrom<String> for AppointmentReviewComment {
    type Error = AppointmentReviewError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(AppointmentReviewError::CommentEmpty);
        }

        if value.chars().count() > 1024 {
            return Err(AppointmentReviewError::CommentTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<AppointmentReviewComment> for String {
    fn from(vo: AppointmentReviewComment) -> Self {
        vo.0
    }
}
