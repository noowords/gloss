use super::value_objects::{ UserId, UserStatus };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    id: UserId,
    status: UserStatus
}

impl User {
    pub fn create(

    ) -> Result<Self, anyhow::Error> {
        let status = UserStatus::try_from("active")?;
        let id = UserId::generate();
        Self::restore(
            id,
            status
        )
    }

    pub fn restore(
        id: UserId,
        status: UserStatus
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            id,
            status
        })
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn status(&self) -> UserStatus {
        self.status.clone()
    }
}
