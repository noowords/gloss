use chrono::NaiveTime;

use super::super::{
    errors::SpecialistScheduleOverrideIntervalError,
    value_objects::{
        SpecialistScheduleOverrideId,
        SpecialistScheduleOverrideIntervalId,
        SpecialistScheduleOverrideIntervalStartsAt,
        SpecialistScheduleOverrideIntervalEndsAt
    }
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleOverrideInterval {
    id: SpecialistScheduleOverrideIntervalId,
    override_id: SpecialistScheduleOverrideId,
    starts_at: SpecialistScheduleOverrideIntervalStartsAt,
    ends_at: SpecialistScheduleOverrideIntervalEndsAt
}

// MARK: Constructors
impl SpecialistScheduleOverrideInterval {
    pub fn create(
        override_id: SpecialistScheduleOverrideId,
        starts_at: SpecialistScheduleOverrideIntervalStartsAt,
        ends_at: SpecialistScheduleOverrideIntervalEndsAt
    ) -> Result<Self, SpecialistScheduleOverrideIntervalError> {
        Self::validate_interval(starts_at, ends_at)?;
        
        let id = SpecialistScheduleOverrideIntervalId::generate();

        Ok(Self { id, override_id, starts_at, ends_at })
    }

    pub fn restore(
        id: SpecialistScheduleOverrideIntervalId,
        override_id: SpecialistScheduleOverrideId,
        starts_at: SpecialistScheduleOverrideIntervalStartsAt,
        ends_at: SpecialistScheduleOverrideIntervalEndsAt
    ) -> Result<Self, SpecialistScheduleOverrideIntervalError> {
        Self::validate_interval(starts_at, ends_at)?;
        
        Ok(Self { id, override_id, starts_at, ends_at })
    }
}

// MARK: Validation
impl SpecialistScheduleOverrideInterval {
    fn validate_interval(
        starts_at: SpecialistScheduleOverrideIntervalStartsAt,
        ends_at: SpecialistScheduleOverrideIntervalEndsAt,
    ) -> Result<(), SpecialistScheduleOverrideIntervalError> {
        let starts_at: NaiveTime = starts_at.into();
        let ends_at: NaiveTime = ends_at.into();

        if starts_at >= ends_at {
            return Err(SpecialistScheduleOverrideIntervalError::InvalidInterval);
        }

        Ok(())
    }
}

// MARK: Getters
impl SpecialistScheduleOverrideInterval {
    pub fn id(&self) -> SpecialistScheduleOverrideIntervalId {
        self.id
    }

    pub fn override_id(&self) -> SpecialistScheduleOverrideId {
        self.override_id
    }

    pub fn starts_at(&self) -> SpecialistScheduleOverrideIntervalStartsAt {
        self.starts_at
    }

    pub fn ends_at(&self) -> SpecialistScheduleOverrideIntervalEndsAt {
        self.ends_at
    }
}
