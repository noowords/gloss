use uuid::{ Uuid };

use domain::user::{
    User,
    profile::{ Profile }
};

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

#[derive(Clone)]
pub struct UserDto {
    pub id: Uuid,
    pub role: String,
    pub phone: Option<String>,
    pub profile: ProfileDto
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto {
            id: user.id().into(),
            role: user.role().clone().into(),
            phone: user.phone().clone().map(|p| p.into()),
            profile: user.profile().into()
        }
    }
}

pub struct GetUserByIdQueryResult(Option<UserDto>);

impl GetUserByIdQueryResult {
    pub fn new(user: Option<UserDto>) -> Self {
        Self(user)
    }

    pub fn value(&self) -> Option<UserDto> {
        self.0.clone()
    }
}

impl From<Option<User>> for GetUserByIdQueryResult {
    fn from(user: Option<User>) -> Self {
        Self::new(user.map(|u| u.into()))
    }
}
