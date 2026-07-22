use chrono::{ NaiveTime };

#[derive(Clone)]
pub struct MasterWorkDay {
    slots: Vec<NaiveTime>
}

impl MasterWorkDay {
    pub fn from_slots(mut slots: Vec<NaiveTime>) -> Self {
        slots.sort();

        Self { slots: slots }
    }

    pub fn slots(&self) -> &Vec<NaiveTime> {
        &self.slots
    }

    pub fn has_slot(&self, time: NaiveTime) -> bool {
        self.slots.contains(&time)
    }

    pub fn add_slot(&mut self,time: NaiveTime) {
        if !self.has_slot(time) {
            self.slots.push(time);
            self.slots.sort();
        }
    }

    pub fn remove_slot(&mut self, time: NaiveTime) {
        self.slots.retain(|t| *t != time);
    }
}
