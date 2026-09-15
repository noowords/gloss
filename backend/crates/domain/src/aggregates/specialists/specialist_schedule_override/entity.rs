use crate::aggregates::specialists::specialist::value_objects::{ SpecialistId };

use super::value_objects::{ SpecialistScheduleOverrideId, SpecialistScheduleOverrideDate, SpecialistScheduleOverrideReason };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleOverride {
    id: SpecialistScheduleOverrideId,
    specialist_id: SpecialistId,
    date: SpecialistScheduleOverrideDate,
    reason: Option<SpecialistScheduleOverrideReason>
}

impl SpecialistScheduleOverride {
    pub fn create(
        specialist_id: SpecialistId,
        date: SpecialistScheduleOverrideDate,
        reason: Option<SpecialistScheduleOverrideReason>
    ) -> Result<Self, anyhow::Error> {
        let id = SpecialistScheduleOverrideId::generate();
        Self::restore(
            id,
            specialist_id,
            date,
            reason
        )
    }

    pub fn restore(
        id: SpecialistScheduleOverrideId,
        specialist_id: SpecialistId,
        date: SpecialistScheduleOverrideDate,
        reason: Option<SpecialistScheduleOverrideReason>
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            id,
            specialist_id,
            date,
            reason
        })
    }

    pub fn id(&self) -> SpecialistScheduleOverrideId {
        self.id
    }

    pub fn specialist_id(&self) -> SpecialistId {
        self.specialist_id
    }

    pub fn date(&self) -> SpecialistScheduleOverrideDate {
        self.date
    }

    pub fn reason(&self) -> Option<SpecialistScheduleOverrideReason> {
        self.reason.clone()
    }
}
