use super::super::value_objects::{
    SalonId,
    SalonScheduleExceptionId,
    SalonScheduleExceptionDate,
    SalonScheduleExceptionType,
    SalonScheduleExceptionReason
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonScheduleException {
    id: SalonScheduleExceptionId,
    salon_id: SalonId,
    date: SalonScheduleExceptionDate,
    r#type: SalonScheduleExceptionType,
    reason: Option<SalonScheduleExceptionReason>
}

// MARK: Constructors
impl SalonScheduleException {
    pub fn create(
        salon_id: SalonId,
        date: SalonScheduleExceptionDate,
        r#type: SalonScheduleExceptionType,
        reason: Option<SalonScheduleExceptionReason>
    ) -> Self {
        let id = SalonScheduleExceptionId::generate();

        Self { id, salon_id, date, r#type, reason }
    }

    pub fn restore(
        id: SalonScheduleExceptionId,
        salon_id: SalonId,
        date: SalonScheduleExceptionDate,
        r#type: SalonScheduleExceptionType,
        reason: Option<SalonScheduleExceptionReason>
    ) -> Self {
        Self { id, salon_id, date, r#type, reason }
    }
}

// MARK: Getters
impl SalonScheduleException {
    pub fn id(&self) -> SalonScheduleExceptionId {
        self.id
    }

    pub fn salon_id(&self) -> SalonId {
        self.salon_id
    }

    pub fn date(&self) -> SalonScheduleExceptionDate {
        self.date
    }

    pub fn r#type(&self) -> SalonScheduleExceptionType {
        self.r#type
    }

    pub fn reason(&self) -> Option<&SalonScheduleExceptionReason> {
        self.reason.as_ref()
    }
}
