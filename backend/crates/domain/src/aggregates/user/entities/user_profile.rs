use super::super::value_objects::{
    UserId,
    UserProfileFirstName,
    UserProfileLastName,
    UserProfileAvatarUrl
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserProfile {
    user_id: UserId,
    first_name: UserProfileFirstName,
    last_name: Option<UserProfileLastName>,
    avatar_url: Option<UserProfileAvatarUrl>
}

// MARK: Constructors
impl UserProfile {
    pub fn create(
        user_id: UserId,
        first_name: UserProfileFirstName,
        last_name: Option<UserProfileLastName>
    ) -> Self {
        let avatar_url = None;

        Self { user_id, first_name, last_name, avatar_url }
    }

    pub fn restore(
        user_id: UserId,
        first_name: UserProfileFirstName,
        last_name: Option<UserProfileLastName>,
        avatar_url: Option<UserProfileAvatarUrl>
    ) -> Self {
        Self { user_id, first_name, last_name, avatar_url }
    }
}

// MARK: Behavior
impl UserProfile {
    pub fn change_first_name(&mut self, first_name: UserProfileFirstName) {
        self.first_name = first_name;
    }

    pub fn change_last_name(&mut self, last_name: Option<UserProfileLastName>) {
        self.last_name = last_name;
    }

    pub fn change_avatar_url(&mut self, avatar_url: Option<UserProfileAvatarUrl>) {
        self.avatar_url = avatar_url;
    }
}

// MARK: Getters
impl UserProfile {
    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn first_name(&self) -> &UserProfileFirstName {
        &self.first_name
    }

    pub fn last_name(&self) -> Option<&UserProfileLastName> {
        self.last_name.as_ref()
    }

    pub fn avatar_url(&self) -> Option<&UserProfileAvatarUrl> {
        self.avatar_url.as_ref()
    }
}
