use domain::aggregates::user::value_objects::{ UserId };

pub struct VerifyOtpCommandResult {
    pub user_id: UserId,
    pub has_profile: bool
}
