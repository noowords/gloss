use crate::aggregates::users::user::value_objects::{ UserId };

use super::value_objects::{ NotificationId, NotificationType, NotificationTitle, NotificationMessage };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notification {
    id: NotificationId,
    user_id: UserId,
    r#type: NotificationType,
    title: NotificationTitle,
    message: NotificationMessage
}

impl Notification {
    pub fn create(
        user_id: UserId,
        r#type: NotificationType,
        title: NotificationTitle,
        message: NotificationMessage
    ) -> Result<Self, anyhow::Error> {
        let id = NotificationId::generate();
        Self::restore(
            id,
            user_id,
            r#type,
            title,
            message
        )
    }

    pub fn restore(
        id: NotificationId,
        user_id: UserId,
        r#type: NotificationType,
        title: NotificationTitle,
        message: NotificationMessage
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            id,
            user_id,
            r#type,
            title,
            message
        })
    }

    pub fn id(&self) -> NotificationId {
        self.id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn r#type(&self) -> NotificationType {
        self.r#type.clone()
    }

    pub fn title(&self) -> NotificationTitle {
        self.title.clone()
    }

    pub fn message(&self) -> NotificationMessage {
        self.message.clone()
    }
}
