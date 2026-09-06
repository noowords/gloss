use crate::contracts::cqrs::command::{ Command };

use super::{ VerifyOtpCommandResult };

#[derive(Clone)]
pub struct VerifyOtpCommand {
    pub provider_type: String,
    pub provider_key: String,
    pub code: String
}

impl Command for VerifyOtpCommand {
    type Result = VerifyOtpCommandResult;
    type Error = anyhow::Error;
}
