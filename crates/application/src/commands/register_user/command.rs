use serde::{ Deserialize };

use super::super::super::buses::command_bus::{ Command };

use super::{ RegisterUserHandler };

#[derive(Deserialize)]
pub struct RegisterUserCommand {
    pub phone: String,
    pub first_name: String,
    pub last_name: String
}

impl Command for RegisterUserCommand {
    type Output = ();
    type Error = anyhow::Error;
    
    type Handler = RegisterUserHandler;
}
