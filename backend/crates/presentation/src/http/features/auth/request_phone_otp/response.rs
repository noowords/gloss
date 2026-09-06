use serde::{ Serialize };

use application::features::auth::commands::request_otp::{ RequestOtpCommandResult };

#[derive(Serialize)]
pub struct RequestPhoneOtpResponse { }

impl From<RequestOtpCommandResult> for RequestPhoneOtpResponse {
    fn from(_result: RequestOtpCommandResult) -> Self {
        Self { }
    }
}
