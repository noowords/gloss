use crate::user::value_objects::{ UserId };
use super::value_objects::{ MasterSchedule };

#[derive(Clone)]
pub struct Master {
    user_id: UserId,
    schedule: MasterSchedule
}

impl Master {
    pub fn create(
        user_id: UserId,
        schedule: MasterSchedule
    ) -> Self {
        Self { user_id, schedule }
    }
    
    pub fn restore(
        user_id: UserId,
        schedule: MasterSchedule
    ) -> Self {
        Self { user_id, schedule }
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn schedule(&self) -> &MasterSchedule {
        &self.schedule
    }
}
