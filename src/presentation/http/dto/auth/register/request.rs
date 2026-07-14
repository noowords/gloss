use serde::{ Deserialize };

use crate::application::commands::register_user::{ RegisterUserCommand };

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub phone: String,
    pub first_name: String,
    pub last_name: String
}

impl From<RegisterUserRequest> for RegisterUserCommand {
    fn from(req: RegisterUserRequest) -> Self {
        Self {
            phone: req.phone,
            first_name: req.first_name,
            last_name: req.last_name
        }
    }
}
