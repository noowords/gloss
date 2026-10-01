use chrono::NaiveTime;

use super::super::{
    errors::SalonScheduleIntervalError,
    value_objects::{
        SalonId,
        SalonScheduleIntervalId,
        SalonScheduleIntervalWeekday,
        SalonScheduleIntervalStartsAt,
        SalonScheduleIntervalEndsAt
    }
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonScheduleInterval {
    id: SalonScheduleIntervalId,
    salon_id: SalonId,
    weekday: SalonScheduleIntervalWeekday,
    starts_at: SalonScheduleIntervalStartsAt,
    ends_at: SalonScheduleIntervalEndsAt
}

// MARK: Constructors
impl SalonScheduleInterval {
    pub fn create(
        salon_id: SalonId,
        weekday: SalonScheduleIntervalWeekday,
        starts_at: SalonScheduleIntervalStartsAt,
        ends_at: SalonScheduleIntervalEndsAt
    ) -> Result<Self, SalonScheduleIntervalError> {
        Self::validate_interval(starts_at, ends_at)?;
        
        let id = SalonScheduleIntervalId::generate();

        Ok(Self { id, salon_id, weekday, starts_at, ends_at })
    }

    pub fn restore(
        id: SalonScheduleIntervalId,
        salon_id: SalonId,
        weekday: SalonScheduleIntervalWeekday,
        starts_at: SalonScheduleIntervalStartsAt,
        ends_at: SalonScheduleIntervalEndsAt
    ) -> Result<Self, SalonScheduleIntervalError> {
        Self::validate_interval(starts_at, ends_at)?;
        
        Ok(Self { id, salon_id, weekday, starts_at, ends_at })
    }
}

// MARK: Validation
impl SalonScheduleInterval {
    fn validate_interval(
        starts_at: SalonScheduleIntervalStartsAt,
        ends_at: SalonScheduleIntervalEndsAt,
    ) -> Result<(), SalonScheduleIntervalError> {
        let starts_at: NaiveTime = starts_at.into();
        let ends_at: NaiveTime = ends_at.into();

        if starts_at >= ends_at {
            return Err(SalonScheduleIntervalError::InvalidInterval);
        }

        Ok(())
    }
}

// MARK: Getters
impl SalonScheduleInterval {
    pub fn id(&self) -> SalonScheduleIntervalId {
        self.id
    }

    pub fn salon_id(&self) -> SalonId {
        self.salon_id
    }

    pub fn weekday(&self) -> SalonScheduleIntervalWeekday {
        self.weekday
    }

    pub fn starts_at(&self) -> SalonScheduleIntervalStartsAt {
        self.starts_at
    }

    pub fn ends_at(&self) -> SalonScheduleIntervalEndsAt {
        self.ends_at
    }
}
