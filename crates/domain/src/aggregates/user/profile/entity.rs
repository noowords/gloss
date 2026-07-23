use crate::aggregates::user::value_objects::{ UserId };

#[derive(Clone)]
pub struct Profile {
    user_id: UserId,
    first_name: String,
    last_name: Option<String>,
    avatar_url: Option<String>,
    bio: Option<String>
}

impl Profile {
    pub fn create(
        user_id: UserId,
        first_name: String,
        last_name: Option<String>,
        avatar_url: Option<String>,
        bio: Option<String>
    ) -> Self {
        Self { user_id, first_name, last_name, avatar_url, bio }
    }
    
    pub fn restore(
        user_id: UserId,
        first_name: String,
        last_name: Option<String>,
        avatar_url: Option<String>,
        bio: Option<String>
    ) -> Self {
        Self { user_id, first_name, last_name, avatar_url, bio }
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn last_name(&self) -> Option<String> {
        self.last_name.clone()
    }

    pub fn avatar_url(&self) -> Option<String> {
        self.avatar_url.clone()
    }

    pub fn bio(&self) -> Option<String> {
        self.bio.clone()
    }
}
