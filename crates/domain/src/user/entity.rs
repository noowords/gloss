use super::profile::{ Profile };
use super::value_objects::{ UserId, UserRole, UserPhone };

pub struct User {
    id: UserId,
    role: UserRole,
    phone: Option<UserPhone>,
    profile: Profile
}

impl User {
    pub fn register(
        phone: Option<UserPhone>,
        first_name: String,
        last_name: Option<String>,
        avatar_url: Option<String>
    ) -> Self {
        let id = UserId::generate();
        
        Self {
            id,
            role: UserRole::Client,
            phone,
            profile: Profile::create(
                id,
                first_name,
                last_name,
                avatar_url,
                None
            )
        }
    }
    
    pub fn restore(
        id: UserId,
        role: UserRole,
        phone: Option<UserPhone>,
        profile: Profile
    ) -> Self {
        Self { id, role, phone, profile }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn role(&self) -> UserRole {
        self.role.clone()
    }

    pub fn phone(&self) -> Option<UserPhone> {
        self.phone.clone()
    }
    
    pub fn profile(&self) -> Profile {
        self.profile.clone()
    }
}
