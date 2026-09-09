use serde::{ Deserialize };

use application::features::auth::commands::send_otp::{ SendOtpCommand };

#[derive(Deserialize)]
pub struct SendOtpRequest {
    pub phone: String
}

impl From<SendOtpRequest> for SendOtpCommand {
    fn from(req: SendOtpRequest) -> Self {
        Self {
            provider_type: "phone".to_string(),
            provider_key: req.phone
        }
    }
}
