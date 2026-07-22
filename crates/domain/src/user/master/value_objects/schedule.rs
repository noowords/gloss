use std::collections::{ HashMap };
use chrono::{ Utc, Weekday, NaiveDate, NaiveTime };

use super::{ MasterWorkDay };

#[derive(Clone)]
pub struct MasterSchedule {
    work_days: HashMap<Weekday, MasterWorkDay>,
    max_advance_booking_days: u32
}

impl MasterSchedule {
    pub fn new(
        work_days: HashMap<Weekday, MasterWorkDay>,
        max_advance_booking_days: u32
    ) -> Self {
        Self { work_days, max_advance_booking_days }
    }

    pub fn work_day(&self, weekday: Weekday) -> Option<&MasterWorkDay> {
        self.work_days.get(&weekday)
    }

    pub fn work_days(&self) -> &HashMap<Weekday, MasterWorkDay> {
        &self.work_days
    }

    pub fn max_advance_booking_days(&self) -> u32 {
        self.max_advance_booking_days
    }

    pub fn is_working_day(&self, weekday: Weekday) -> bool {
        self.work_days.contains_key(&weekday)
    }

    pub fn get_slots(&self, weekday: Weekday) -> Vec<NaiveTime> {
        self.work_day(weekday)
            .map(|wd| wd.slots().clone())
            .unwrap_or_default()
    }

    pub fn add_work_day(&mut self, weekday: Weekday, slots: Vec<NaiveTime>) {
        self.work_days.insert(
            weekday,
            MasterWorkDay::from_slots(slots)
        );
    }

    pub fn is_within_advance_limit(&self, date: NaiveDate) -> bool {
        let today = Utc::now().date_naive();
        let days_ahead = (date - today).num_days();

        days_ahead >= 0 && days_ahead <= self.max_advance_booking_days as i64
    }
}
