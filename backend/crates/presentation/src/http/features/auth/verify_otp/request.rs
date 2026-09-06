use serde::{ Deserialize };

use application::features::auth::commands::verify_otp::{ VerifyOtpCommand };

#[derive(Deserialize)]
pub struct VerifyOtpRequest {
    pub phone: String,
    pub code: String
}

impl From<VerifyOtpRequest> for VerifyOtpCommand {
    fn from(req: VerifyOtpRequest) -> Self {
        Self {
            provider_type: "phone".to_string(),
            provider_key: req.phone,
            code: req.code
        }
    }
}
