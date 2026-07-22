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

pub struct GetUsersQueryResult(Vec<UserDto>);

impl GetUsersQueryResult {
    pub fn new(users: Vec<UserDto>) -> Self {
        Self(users)
    }

    pub fn value(&self) -> Vec<UserDto> {
        self.0.clone()
    }
}

impl From<Vec<User>> for GetUsersQueryResult {
    fn from(users: Vec<User>) -> Self {
        Self::new(users.into_iter().map(|u| u.into()).collect())
    }
}
