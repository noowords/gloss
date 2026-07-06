use serde::{ Deserialize };

#[derive(Deserialize)]
pub struct CreateUserCommand {
    pub role: Option<String>,
    pub phone: Option<String>
}
