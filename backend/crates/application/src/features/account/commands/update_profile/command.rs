use domain::aggregates::{
    user::value_objects::{ UserId },
    profile::value_objects::{ ProfileFirstName, ProfileLastName, ProfileAvatarUrl, ProfileBio }
};

use crate::contracts::cqrs::command::{ Command };

use super::{ UpdateAccountProfileCommandResult };

#[derive(Clone)]
pub struct UpdateAccountProfileCommand {
    pub user_id: UserId,
    pub first_name: ProfileFirstName,
    pub last_name: Option<ProfileLastName>,
    pub avatar_url: Option<ProfileAvatarUrl>,
    pub bio: Option<ProfileBio>
}

impl Command for UpdateAccountProfileCommand {
    type Result = UpdateAccountProfileCommandResult;
    type Error = anyhow::Error;
}
