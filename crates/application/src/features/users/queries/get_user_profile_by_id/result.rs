use domain::aggregates::profile::{ Profile };

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
            first_name: profile.first_name().into(),
            last_name: profile.last_name().map(String::from),
            avatar_url: profile.avatar_url().map(String::from),
            bio: profile.bio().map(String::from)
        }
    }
}

pub struct GetUserProfileByIdQueryResult(Option<ProfileDto>);

impl From<Option<Profile>> for GetUserProfileByIdQueryResult {
    fn from(profile: Option<Profile>) -> Self {
        Self(profile.map(|p| p.into()))
    }
}
