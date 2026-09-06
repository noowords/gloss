use crate::aggregates::user::value_objects::{ UserId };

use super::value_objects::{ ProfileFirstName, ProfileLastName, ProfileAvatarUrl, ProfileBio };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    user_id: UserId,
    first_name: ProfileFirstName,
    last_name: Option<ProfileLastName>,
    avatar_url: Option<ProfileAvatarUrl>,
    bio: Option<ProfileBio>
}

impl Profile {
    pub fn create(
        user_id: UserId,
        first_name: ProfileFirstName,
        last_name: Option<ProfileLastName>,
        avatar_url: Option<ProfileAvatarUrl>,
        bio: Option<ProfileBio>
    ) -> Self {
        Self { user_id, first_name, last_name, avatar_url, bio }
    }
    
    pub fn restore(
        user_id: UserId,
        first_name: ProfileFirstName,
        last_name: Option<ProfileLastName>,
        avatar_url: Option<ProfileAvatarUrl>,
        bio: Option<ProfileBio>
    ) -> Self {
        Self { user_id, first_name, last_name, avatar_url, bio }
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn first_name(&self) -> ProfileFirstName {
        self.first_name.clone()
    }

    pub fn last_name(&self) -> Option<ProfileLastName> {
        self.last_name.clone()
    }

    pub fn avatar_url(&self) -> Option<ProfileAvatarUrl> {
        self.avatar_url.clone()
    }

    pub fn bio(&self) -> Option<ProfileBio> {
        self.bio.clone()
    }
}
