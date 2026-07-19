use super::value_objects::{ UserId, UserRole, UserPhone };

#[derive(Clone, Eq, PartialEq)]
pub struct User {
    id: UserId,
    role: UserRole,
    phone: Option<UserPhone>
}

impl User {
    pub fn new(
        id: Option<UserId>,
        role: Option<UserRole>,
        phone: Option<UserPhone>
    ) -> Self {
        Self {
            id: id.unwrap_or(UserId::new()),
            role: role.unwrap_or(UserRole::Client),
            phone
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn role(&self) -> &UserRole {
        &self.role
    }

    pub fn phone(&self) -> &Option<UserPhone> {
        &self.phone
    }
}
