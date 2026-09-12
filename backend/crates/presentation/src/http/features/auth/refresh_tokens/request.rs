use serde::{ Deserialize };

use application::features::auth::commands::refresh_tokens::{ RefreshTokensCommand };

#[derive(Deserialize)]
pub struct RefreshTokensRequest {
    pub refresh_token: String
}

impl From<RefreshTokensRequest> for RefreshTokensCommand {
    fn from(req: RefreshTokensRequest) -> Self {
        Self {
            refresh_token: req.refresh_token
        }
    }
}
