use chrono::NaiveDateTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AppointmentReviewCreatedAt(NaiveDateTime);

// MARK: Conversions
impl From<NaiveDateTime> for AppointmentReviewCreatedAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<AppointmentReviewCreatedAt> for NaiveDateTime {
    fn from(vo: AppointmentReviewCreatedAt) -> Self {
        vo.0
    }
}
