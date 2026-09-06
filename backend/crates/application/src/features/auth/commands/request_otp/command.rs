use crate::contracts::cqrs::command::{ Command };

use super::{ RequestOtpCommandResult };

#[derive(Clone)]
pub struct RequestOtpCommand {
    pub provider_type: String,
    pub provider_key: String
}

impl Command for RequestOtpCommand {
    type Result = RequestOtpCommandResult;
    type Error = anyhow::Error;
}
