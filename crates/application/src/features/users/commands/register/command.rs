use crate::contracts::cqrs::command::{ Command };

#[derive(Clone)]
pub struct RegisterUserCommand {
    pub phone: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>
}

impl Command for RegisterUserCommand {
    type Error = anyhow::Error;
}
