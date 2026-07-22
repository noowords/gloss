use serde::{ Deserialize };

use crate::buses::command_bus::{ Command };
use crate::persistence::{
    command_results::{ RegisterUserCommandResult },
    command_handlers::{ RegisterUserCommandHandler },
};

#[derive(Clone, Deserialize)]
pub struct RegisterUserCommand {
    pub phone: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>
}

impl Command for RegisterUserCommand {
    type Result = RegisterUserCommandResult;
    type Error = anyhow::Error;

    type Handler = RegisterUserCommandHandler;
}
