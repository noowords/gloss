use uuid::{ Uuid };

use super::{ Profile };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Specialist {
    pub user_id: Uuid,
    pub profile: Profile
}
