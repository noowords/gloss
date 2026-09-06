use serde::{ Deserialize };

use application::features::auth::commands::request_otp::{ RequestOtpCommand };

#[derive(Deserialize)]
pub struct RequestPhoneOtpRequest {
    pub phone: String
}

impl From<RequestPhoneOtpRequest> for RequestOtpCommand {
    fn from(req: RequestPhoneOtpRequest) -> Self {
        Self {
            provider_type: "phone".to_string(),
            provider_key: req.phone
        }
    }
}
