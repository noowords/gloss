use super::super::value_objects::{
    SpecialistId,
    SpecialistScheduleOverrideId,
    SpecialistScheduleOverrideDate,
    SpecialistScheduleOverrideReason
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleOverride {
    id: SpecialistScheduleOverrideId,
    specialist_id: SpecialistId,
    date: SpecialistScheduleOverrideDate,
    reason: Option<SpecialistScheduleOverrideReason>
}

// MARK: Constructors
impl SpecialistScheduleOverride {
    pub fn create(
        specialist_id: SpecialistId,
        date: SpecialistScheduleOverrideDate,
        reason: Option<SpecialistScheduleOverrideReason>
    ) -> Self {
        let id = SpecialistScheduleOverrideId::generate();

        Self { id, specialist_id, date,  reason }
    }

    pub fn restore(
        id: SpecialistScheduleOverrideId,
        specialist_id: SpecialistId,
        date: SpecialistScheduleOverrideDate,
        reason: Option<SpecialistScheduleOverrideReason>
    ) -> Self {
        Self { id, specialist_id, date,  reason }
    }
}

// MARK: Getters
impl SpecialistScheduleOverride {
    pub fn id(&self) -> SpecialistScheduleOverrideId {
        self.id
    }

    pub fn specialist_id(&self) -> SpecialistId {
        self.specialist_id
    }

    pub fn date(&self) -> SpecialistScheduleOverrideDate {
        self.date
    }

    pub fn reason(&self) -> Option<&SpecialistScheduleOverrideReason> {
        self.reason.as_ref()
    }
}
