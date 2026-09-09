use crate::contracts::cqrs::command::{ Command };

use super::{ SendOtpCommandResult };

#[derive(Clone)]
pub struct SendOtpCommand {
    pub provider_type: String,
    pub provider_key: String
}

impl Command for SendOtpCommand {
    type Result = SendOtpCommandResult;
    type Error = anyhow::Error;
}
