use chrono::NaiveDate;

use super::super::{
    errors::SpecialistScheduleError,
    value_objects::{
        SpecialistId,
        SpecialistScheduleId,
        SpecialistScheduleEffectiveFrom,
        SpecialistScheduleEffectiveUntil
    }
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistSchedule {
    id: SpecialistScheduleId,
    specialist_id: SpecialistId,
    effective_from: SpecialistScheduleEffectiveFrom,
    effective_until: Option<SpecialistScheduleEffectiveUntil>
}

// MARK: Constructors
impl SpecialistSchedule {
    pub fn create(
        specialist_id: SpecialistId,
        effective_from: SpecialistScheduleEffectiveFrom,
        effective_until: Option<SpecialistScheduleEffectiveUntil>
    ) -> Result<Self, SpecialistScheduleError> {
        Self::validate_effective_period(effective_from, effective_until)?;
        
        let id = SpecialistScheduleId::generate();

        Ok(Self { id, specialist_id, effective_from, effective_until })
    }

    pub fn restore(
        id: SpecialistScheduleId,
        specialist_id: SpecialistId,
        effective_from: SpecialistScheduleEffectiveFrom,
        effective_until: Option<SpecialistScheduleEffectiveUntil>
    ) -> Result<Self, SpecialistScheduleError> {
        Self::validate_effective_period(effective_from, effective_until)?;
        
        Ok(Self { id, specialist_id, effective_from, effective_until })
    }
}

// MARK: Validation
impl SpecialistSchedule {
    fn validate_effective_period(
        effective_from: SpecialistScheduleEffectiveFrom,
        effective_until: Option<SpecialistScheduleEffectiveUntil>,
    ) -> Result<(), SpecialistScheduleError> {
        if let Some(effective_until) = effective_until {
            let effective_from: NaiveDate = effective_from.into();
            let effective_until: NaiveDate = effective_until.into();

            if effective_until < effective_from {
                return Err(SpecialistScheduleError::InvalidEffectivePeriod);
            }
        }

        Ok(())
    }
}

// MARK: Behavior
impl SpecialistSchedule {
    pub fn close(
        &mut self,
        effective_until: SpecialistScheduleEffectiveUntil
    ) -> Result<(), SpecialistScheduleError> {
        if self.effective_until.is_some() {
            return Err(SpecialistScheduleError::AlreadyClosed);
        }

        Self::validate_effective_period(
            self.effective_from,
            Some(effective_until),
        )?;

        self.effective_until = Some(effective_until);

        Ok(())
    }
}

// MARK: Getters
impl SpecialistSchedule {
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

    pub fn is_open(&self) -> bool {
        self.effective_until.is_none()
    }
}
