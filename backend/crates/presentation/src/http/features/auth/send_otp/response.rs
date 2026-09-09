use serde::{ Serialize };

use application::features::auth::commands::send_otp::{ SendOtpCommandResult };

#[derive(Serialize)]
pub struct SendOtpResponse { }

impl From<SendOtpCommandResult> for SendOtpResponse {
    fn from(_result: SendOtpCommandResult) -> Self {
        Self { }
    }
}
