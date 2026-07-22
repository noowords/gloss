use crate::{ Command };
use super::{ RegisterUserCommandResult, RegisterUserCommandHandler };

#[derive(Clone)]
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
