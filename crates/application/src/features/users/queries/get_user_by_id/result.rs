use uuid::{ Uuid };

use domain::aggregates::{
    user::{ User },
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
            first_name: profile.first_name().into(),
            last_name: profile.last_name().map(String::from),
            avatar_url: profile.avatar_url().map(String::from),
            bio: profile.bio().map(String::from)
        }
    }
}

#[derive(Clone)]
pub struct UserDto {
    pub id: Uuid,
    pub role: String
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto {
            id: user.id().into(),
            role: user.role().clone().into()
        }
    }
}

pub struct GetUserByIdQueryResult(Option<UserDto>);

impl From<Option<User>> for GetUserByIdQueryResult {
    fn from(user: Option<User>) -> Self {
        Self(user.map(|u| u.into()))
    }
}
