use serde::{ Deserialize };

use crate::application::commands::create_user::{ CreateUserCommand };

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub role: Option<String>,
    pub phone: Option<String>
}

impl From<CreateUserRequest> for CreateUserCommand {
    fn from(req: CreateUserRequest) -> Self {
        Self { role: req.role, phone: req.phone }
    }
}
