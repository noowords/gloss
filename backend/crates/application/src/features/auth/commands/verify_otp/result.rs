use domain::aggregates::user::value_objects::{ UserId };

pub struct VerifyOtpCommandResult {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: UserId,
    pub has_profile: bool
}
