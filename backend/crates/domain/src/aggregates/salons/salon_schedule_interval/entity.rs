use crate::aggregates::salons::salon::value_objects::{ SalonId };

use super::value_objects::{ SalonScheduleIntervalId, SalonScheduleIntervalWeekday, SalonScheduleIntervalStartsAt, SalonScheduleIntervalEndsAt };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonScheduleInterval {
    id: SalonScheduleIntervalId,
    salon_id: SalonId,
    weekday: SalonScheduleIntervalWeekday,
    starts_at: SalonScheduleIntervalStartsAt,
    ends_at: SalonScheduleIntervalEndsAt
}

impl SalonScheduleInterval {
    pub fn create(
        salon_id: SalonId,
        weekday: SalonScheduleIntervalWeekday,
        starts_at: SalonScheduleIntervalStartsAt,
        ends_at: SalonScheduleIntervalEndsAt
    ) -> Result<Self, anyhow::Error> {
        let id = SalonScheduleIntervalId::generate();
        Self::restore(
            id,
            salon_id,
            weekday,
            starts_at,
            ends_at
        )
    }

    pub fn restore(
        id: SalonScheduleIntervalId,
        salon_id: SalonId,
        weekday: SalonScheduleIntervalWeekday,
        starts_at: SalonScheduleIntervalStartsAt,
        ends_at: SalonScheduleIntervalEndsAt
    ) -> Result<Self, anyhow::Error> {
        if chrono::NaiveTime::from(starts_at) >= chrono::NaiveTime::from(ends_at) {
            anyhow::bail!("The interval must end after it starts");
        }

        Ok(Self {
            id,
            salon_id,
            weekday,
            starts_at,
            ends_at
        })
    }

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
