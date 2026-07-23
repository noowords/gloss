use serde::{ Deserialize };

use application::features::users::commands::register_user::{ RegisterUserCommand };

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub phone: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>
}

impl From<RegisterUserRequest> for RegisterUserCommand {
    fn from(req: RegisterUserRequest) -> Self {
        Self {
            phone: req.phone,
            first_name: req.first_name,
            last_name: req.last_name,
            avatar_url: req.avatar_url
        }
    }
}
