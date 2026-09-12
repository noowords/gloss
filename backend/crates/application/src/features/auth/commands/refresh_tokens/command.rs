use crate::contracts::cqrs::command::{ Command };

use super::{ RefreshTokensCommandResult };

#[derive(Clone)]
pub struct RefreshTokensCommand {
    pub refresh_token: String
}

impl Command for RefreshTokensCommand {
    type Result = RefreshTokensCommandResult;
    type Error = anyhow::Error;
}
