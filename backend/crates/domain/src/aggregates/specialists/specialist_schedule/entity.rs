use crate::aggregates::specialists::specialist::value_objects::{ SpecialistId };

use super::value_objects::{ SpecialistScheduleId, SpecialistScheduleEffectiveFrom, SpecialistScheduleEffectiveUntil };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistSchedule {
    id: SpecialistScheduleId,
    specialist_id: SpecialistId,
    effective_from: SpecialistScheduleEffectiveFrom,
    effective_until: Option<SpecialistScheduleEffectiveUntil>
}

impl SpecialistSchedule {
    pub fn create(
        specialist_id: SpecialistId,
        effective_from: SpecialistScheduleEffectiveFrom,
        effective_until: Option<SpecialistScheduleEffectiveUntil>
    ) -> Result<Self, anyhow::Error> {
        let id = SpecialistScheduleId::generate();
        Self::restore(
            id,
            specialist_id,
            effective_from,
            effective_until
        )
    }

    pub fn restore(
        id: SpecialistScheduleId,
        specialist_id: SpecialistId,
        effective_from: SpecialistScheduleEffectiveFrom,
        effective_until: Option<SpecialistScheduleEffectiveUntil>
    ) -> Result<Self, anyhow::Error> {
        if effective_until.is_some_and(|until| chrono::NaiveDate::from(until) < chrono::NaiveDate::from(effective_from)) {
            anyhow::bail!("The schedule must end on or after its effective date");
        }

        Ok(Self {
            id,
            specialist_id,
            effective_from,
            effective_until
        })
    }

    pub fn id(&self) -> SpecialistScheduleId {
        self.id
    }

    pub fn specialist_id(&self) -> SpecialistId {
        self.specialist_id
    }

    pub fn effective_from(&self) -> SpecialistScheduleEffectiveFrom {
        self.effective_from
    }

    pub fn effective_until(&self) -> Option<SpecialistScheduleEffectiveUntil> {
        self.effective_until
    }
}
