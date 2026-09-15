use crate::aggregates::salons::salon::value_objects::{ SalonId };

use super::value_objects::{ SalonScheduleExceptionId, SalonScheduleExceptionDate, SalonScheduleExceptionType, SalonScheduleExceptionReason };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonScheduleException {
    id: SalonScheduleExceptionId,
    salon_id: SalonId,
    date: SalonScheduleExceptionDate,
    r#type: SalonScheduleExceptionType,
    reason: Option<SalonScheduleExceptionReason>
}

impl SalonScheduleException {
    pub fn create(
        salon_id: SalonId,
        date: SalonScheduleExceptionDate,
        r#type: SalonScheduleExceptionType,
        reason: Option<SalonScheduleExceptionReason>
    ) -> Result<Self, anyhow::Error> {
        let id = SalonScheduleExceptionId::generate();
        Self::restore(
            id,
            salon_id,
            date,
            r#type,
            reason
        )
    }

    pub fn restore(
        id: SalonScheduleExceptionId,
        salon_id: SalonId,
        date: SalonScheduleExceptionDate,
        r#type: SalonScheduleExceptionType,
        reason: Option<SalonScheduleExceptionReason>
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            id,
            salon_id,
            date,
            r#type,
            reason
        })
    }

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
        self.r#type.clone()
    }

    pub fn reason(&self) -> Option<SalonScheduleExceptionReason> {
        self.reason.clone()
    }
}
