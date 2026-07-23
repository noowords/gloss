use domain::aggregates::user::profile::{ Profile };

#[derive(Clone)]
pub struct ProfileDto {
    pub first_name: String,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>
}

impl From<Profile> for ProfileDto {
    fn from(profile: Profile) -> Self {
        ProfileDto {
            first_name: profile.first_name().clone(),
            last_name: profile.last_name().clone(),
            avatar_url: profile.avatar_url().clone(),
            bio: profile.bio().clone()
        }
    }
}

pub struct GetUserProfileByIdQueryResult(Option<ProfileDto>);

impl GetUserProfileByIdQueryResult {
    pub fn new(profile: Option<ProfileDto>) -> Self {
        Self(profile)
    }

    pub fn value(&self) -> Option<ProfileDto> {
        self.0.clone()
    }
}

impl From<Option<Profile>> for GetUserProfileByIdQueryResult {
    fn from(profile: Option<Profile>) -> Self {
        Self(profile.map(|p| p.into()))
    }
}
