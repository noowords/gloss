use chrono::NaiveTime;

use super::super::{
    errors::SpecialistScheduleIntervalError,
    value_objects::{
        SpecialistScheduleId,
        SpecialistScheduleIntervalId,
        SpecialistScheduleIntervalWeekday,
        SpecialistScheduleIntervalStartsAt,
        SpecialistScheduleIntervalEndsAt
    }
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleInterval {
    id: SpecialistScheduleIntervalId,
    schedule_id: SpecialistScheduleId,
    weekday: SpecialistScheduleIntervalWeekday,
    starts_at: SpecialistScheduleIntervalStartsAt,
    ends_at: SpecialistScheduleIntervalEndsAt
}

// MARK: Constructors
impl SpecialistScheduleInterval {
    pub fn create(
        schedule_id: SpecialistScheduleId,
        weekday: SpecialistScheduleIntervalWeekday,
        starts_at: SpecialistScheduleIntervalStartsAt,
        ends_at: SpecialistScheduleIntervalEndsAt
    ) -> Result<Self, SpecialistScheduleIntervalError> {
        Self::validate_interval(starts_at, ends_at)?;
        
        let id = SpecialistScheduleIntervalId::generate();

        Ok(Self { id, schedule_id, weekday, starts_at, ends_at })
    }

    pub fn restore(
        id: SpecialistScheduleIntervalId,
        schedule_id: SpecialistScheduleId,
        weekday: SpecialistScheduleIntervalWeekday,
        starts_at: SpecialistScheduleIntervalStartsAt,
        ends_at: SpecialistScheduleIntervalEndsAt
    ) -> Result<Self, SpecialistScheduleIntervalError> {
        Self::validate_interval(starts_at, ends_at)?;
        
        Ok(Self { id, schedule_id, weekday, starts_at, ends_at })
    }
}

// MARK: Validation
impl SpecialistScheduleInterval {
    fn validate_interval(
        starts_at: SpecialistScheduleIntervalStartsAt,
        ends_at: SpecialistScheduleIntervalEndsAt,
    ) -> Result<(), SpecialistScheduleIntervalError> {
        let starts_at: NaiveTime = starts_at.into();
        let ends_at: NaiveTime = ends_at.into();

        if starts_at >= ends_at {
            return Err(SpecialistScheduleIntervalError::InvalidInterval);
        }

        Ok(())
    }
}

// MARK: Getters
impl SpecialistScheduleInterval {
    pub fn id(&self) -> SpecialistScheduleIntervalId {
        self.id
    }

    pub fn schedule_id(&self) -> SpecialistScheduleId {
        self.schedule_id
    }

    pub fn weekday(&self) -> SpecialistScheduleIntervalWeekday {
        self.weekday
    }

    pub fn starts_at(&self) -> SpecialistScheduleIntervalStartsAt {
        self.starts_at
    }

    pub fn ends_at(&self) -> SpecialistScheduleIntervalEndsAt {
        self.ends_at
    }
}
