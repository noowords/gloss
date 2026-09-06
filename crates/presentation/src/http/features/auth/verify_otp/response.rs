use uuid::{ Uuid };
use serde::{ Serialize };

use application::features::auth::commands::verify_otp::{ VerifyOtpCommandResult };

#[derive(Serialize)]
pub struct VerifyOtpResponse {
    pub user_id: Uuid,
    pub has_profile: bool
}

impl From<VerifyOtpCommandResult> for VerifyOtpResponse {
    fn from(result: VerifyOtpCommandResult) -> Self {
        Self {
            user_id: result.user_id.into(),
            has_profile: result.has_profile
        }
    }
}
