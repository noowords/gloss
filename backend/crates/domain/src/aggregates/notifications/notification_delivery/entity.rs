use crate::aggregates::notifications::notification::value_objects::{ NotificationId };

use super::value_objects::{
    NotificationDeliveryId,
    NotificationDeliveryChannel,
    NotificationDeliveryStatus,
    NotificationDeliveryAttemptedAt,
    NotificationDeliveryDeliveredAt,
    NotificationDeliveryReadAt,
    NotificationDeliveryError
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationDelivery {
    id: NotificationDeliveryId,
    notification_id: NotificationId,
    channel: NotificationDeliveryChannel,
    status: NotificationDeliveryStatus,
    attempted_at: Option<NotificationDeliveryAttemptedAt>,
    delivered_at: Option<NotificationDeliveryDeliveredAt>,
    read_at: Option<NotificationDeliveryReadAt>,
    error: Option<NotificationDeliveryError>
}

impl NotificationDelivery {
    pub fn create(
        notification_id: NotificationId,
        channel: NotificationDeliveryChannel
    ) -> Result<Self, anyhow::Error> {
        Self::restore(
            NotificationDeliveryId::generate(),
            notification_id,
            channel,
            NotificationDeliveryStatus::Pending,
            None,
            None,
            None,
            None
        )
    }

    pub fn restore(
        id: NotificationDeliveryId,
        notification_id: NotificationId,
        channel: NotificationDeliveryChannel,
        status: NotificationDeliveryStatus,
        attempted_at: Option<NotificationDeliveryAttemptedAt>,
        delivered_at: Option<NotificationDeliveryDeliveredAt>,
        read_at: Option<NotificationDeliveryReadAt>,
        error: Option<NotificationDeliveryError>
    ) -> Result<Self, anyhow::Error> {
        Ok(Self { id, notification_id, channel, status, attempted_at, delivered_at, read_at, error })
    }

    pub fn id(&self) -> NotificationDeliveryId { self.id }
    pub fn notification_id(&self) -> NotificationId { self.notification_id }
    pub fn channel(&self) -> NotificationDeliveryChannel { self.channel.clone() }
    pub fn status(&self) -> NotificationDeliveryStatus { self.status }
    pub fn attempted_at(&self) -> Option<NotificationDeliveryAttemptedAt> { self.attempted_at }
    pub fn delivered_at(&self) -> Option<NotificationDeliveryDeliveredAt> { self.delivered_at }
    pub fn read_at(&self) -> Option<NotificationDeliveryReadAt> { self.read_at }
    pub fn error(&self) -> Option<NotificationDeliveryError> { self.error.clone() }
}
