use uuid::{ Uuid };
use serde::{ Serialize };

use application::features::auth::commands::refresh_tokens::{ RefreshTokensCommandResult };

#[derive(Serialize)]
pub struct RefreshTokensResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: Uuid,
    pub has_profile: bool
}

impl From<RefreshTokensCommandResult> for RefreshTokensResponse {
    fn from(result: RefreshTokensCommandResult) -> Self {
        Self {
            access_token: result.access_token,
            refresh_token: result.refresh_token,
            user_id: result.user_id.into(),
            has_profile: result.has_profile
        }
    }
}
