use domain::aggregates::{
    users::user::value_objects::{ UserId },
    users::profile::value_objects::{ ProfileFirstName, ProfileLastName, ProfileAvatarUrl }
};

use crate::contracts::cqrs::command::{ Command };

use super::{ CreateAccountProfileCommandResult };

#[derive(Clone)]
pub struct CreateAccountProfileCommand {
    pub user_id: UserId,
    pub first_name: ProfileFirstName,
    pub last_name: Option<ProfileLastName>,
    pub avatar_url: Option<ProfileAvatarUrl>
}

impl Command for CreateAccountProfileCommand {
    type Result = CreateAccountProfileCommandResult;
    type Error = anyhow::Error;
}
