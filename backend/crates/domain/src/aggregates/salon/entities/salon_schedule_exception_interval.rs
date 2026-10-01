use chrono::NaiveTime;

use super::super::{
    errors::SalonScheduleExceptionIntervalError,
    value_objects::{
        SalonScheduleExceptionId,
        SalonScheduleExceptionIntervalId,
        SalonScheduleExceptionIntervalStartsAt,
        SalonScheduleExceptionIntervalEndsAt
    }
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonScheduleExceptionInterval {
    id: SalonScheduleExceptionIntervalId,
    exception_id: SalonScheduleExceptionId,
    starts_at: SalonScheduleExceptionIntervalStartsAt,
    ends_at: SalonScheduleExceptionIntervalEndsAt
}

// MARK: Constructors
impl SalonScheduleExceptionInterval {
    pub fn create(
        exception_id: SalonScheduleExceptionId,
        starts_at: SalonScheduleExceptionIntervalStartsAt,
        ends_at: SalonScheduleExceptionIntervalEndsAt
    ) -> Result<Self, SalonScheduleExceptionIntervalError> {
        Self::validate_interval(starts_at, ends_at)?;
        
        let id = SalonScheduleExceptionIntervalId::generate();

        Ok(Self { id, exception_id, starts_at, ends_at })
    }

    pub fn restore(
        id: SalonScheduleExceptionIntervalId,
        exception_id: SalonScheduleExceptionId,
        starts_at: SalonScheduleExceptionIntervalStartsAt,
        ends_at: SalonScheduleExceptionIntervalEndsAt
    ) -> Result<Self, SalonScheduleExceptionIntervalError> {
        Self::validate_interval(starts_at, ends_at)?;
        
        Ok(Self { id, exception_id, starts_at, ends_at })
    }
}

// MARK: Validation
impl SalonScheduleExceptionInterval {
    fn validate_interval(
        starts_at: SalonScheduleExceptionIntervalStartsAt,
        ends_at: SalonScheduleExceptionIntervalEndsAt,
    ) -> Result<(), SalonScheduleExceptionIntervalError> {
        let starts_at: NaiveTime = starts_at.into();
        let ends_at: NaiveTime = ends_at.into();

        if starts_at >= ends_at {
            return Err(SalonScheduleExceptionIntervalError::InvalidInterval);
        }

        Ok(())
    }
}

// MARK: Getters
impl SalonScheduleExceptionInterval {
    pub fn id(&self) -> SalonScheduleExceptionIntervalId {
        self.id
    }

    pub fn exception_id(&self) -> SalonScheduleExceptionId {
        self.exception_id
    }

    pub fn starts_at(&self) -> SalonScheduleExceptionIntervalStartsAt {
        self.starts_at
    }

    pub fn ends_at(&self) -> SalonScheduleExceptionIntervalEndsAt {
        self.ends_at
    }
}
