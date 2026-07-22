use serde::{ Serialize };

use application::projections::get_user_profile_by_id::{ GetUserProfileByIdQueryResult };

#[derive(Serialize)]
pub struct Profile {
    pub first_name: String,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>
}

#[derive(Serialize)]
pub struct GetUserProfileByIdResponse(pub Option<Profile>);

impl From<GetUserProfileByIdQueryResult> for GetUserProfileByIdResponse {
    fn from(output: GetUserProfileByIdQueryResult) -> Self {
        Self(output.value().map(|profile| Profile {
            first_name: profile.first_name,
            last_name: profile.last_name,
            avatar_url: profile.avatar_url,
            bio: profile.bio
        }))
    }
}
