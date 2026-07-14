use serde::{ Deserialize };

#[derive(Deserialize)]
pub struct RegisterUserCommand {
    pub phone: String,
    pub first_name: String,
    pub last_name: String
}
