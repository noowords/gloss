use chrono::NaiveDateTime;

use crate::aggregates::user::value_objects::UserId;

use super::super::{
    errors::NotificationError,
    value_objects::{
        NotificationId,
        NotificationType,
        NotificationTitle,
        NotificationMessage,
        NotificationReadAt,
        NotificationCreatedAt
    }
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notification {
    id: NotificationId,
    user_id: UserId,
    r#type: NotificationType,
    title: NotificationTitle,
    message: NotificationMessage,
    read_at: Option<NotificationReadAt>,
    created_at: NotificationCreatedAt
}

// MARK: Constructors
impl Notification {
    pub fn create(
        user_id: UserId,
        r#type: NotificationType,
        title: NotificationTitle,
        message: NotificationMessage,
        created_at: NotificationCreatedAt
    ) -> Self {
        let id = NotificationId::generate();
        let read_at = None;

        Self { id, user_id, r#type, title, message, read_at, created_at }
    }

    pub fn restore(
        id: NotificationId,
        user_id: UserId,
        r#type: NotificationType,
        title: NotificationTitle,
        message: NotificationMessage,
        read_at: Option<NotificationReadAt>,
        created_at: NotificationCreatedAt
    ) -> Result<Self, NotificationError> {
        Self::validate_read_time(created_at, read_at)?;
        
        Ok(Self { id, user_id, r#type, title, message, read_at, created_at })
    }
}

// MARK: Validation
impl Notification {
    fn validate_read_time(
        created_at: NotificationCreatedAt,
        read_at: Option<NotificationReadAt>
    ) -> Result<(), NotificationError> {
        let Some(read_at) = read_at else {
            return Ok(());
        };

        let created_at: NaiveDateTime = created_at.into();
        let read_at: NaiveDateTime = read_at.into();

        if read_at < created_at {
            return Err(NotificationError::InvalidReadTime);
        }

        Ok(())
    }
}

// MARK: Behavior
impl Notification {
    pub fn mark_as_read(
        &mut self,
        read_at: NotificationReadAt
    ) -> Result<(), NotificationError> {
        if self.read_at.is_some() {
            return Err(NotificationError::AlreadyRead);
        }

        Self::validate_read_time(self.created_at, Some(read_at))?;

        self.read_at = Some(read_at);

        Ok(())
    }
}

// MARK: Getters
impl Notification {
    pub fn id(&self) -> NotificationId {
        self.id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn r#type(&self) -> NotificationType {
        self.r#type
    }

    pub fn title(&self) -> &NotificationTitle {
        &self.title
    }

    pub fn message(&self) -> &NotificationMessage {
        &self.message
    }

    pub fn read_at(&self) -> Option<NotificationReadAt> {
        self.read_at
    }
    
    pub fn created_at(&self) -> NotificationCreatedAt {
        self.created_at
    }

    pub fn is_read(&self) -> bool {
        self.read_at.is_some()
    }
}
