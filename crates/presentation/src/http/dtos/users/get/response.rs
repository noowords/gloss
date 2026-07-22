use serde::{ Serialize };
use uuid::{ Uuid };

use application::features::queries::get_users::{ GetUsersQueryResult };

#[derive(Serialize)]
pub struct Profile {
    pub first_name: String,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>
}

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub role: String,
    pub phone: Option<String>,
    pub profile: Profile
}

#[derive(Serialize)]
pub struct GetUsersResponse(pub Vec<User>);

impl From<GetUsersQueryResult> for GetUsersResponse {
    fn from(output: GetUsersQueryResult) -> Self {
        Self(output.value().into_iter().map(|user| User {
            id: user.id,
            role: user.role,
            phone: user.phone,
            profile: Profile {
                first_name: user.profile.first_name,
                last_name: user.profile.last_name,
                avatar_url: user.profile.avatar_url,
                bio: user.profile.bio
            }
        }).collect())
    }
}
