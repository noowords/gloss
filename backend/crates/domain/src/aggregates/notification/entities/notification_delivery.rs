use chrono::NaiveDateTime;

use super::super::{
    errors::NotificationDeliveryError,
    value_objects::{
        NotificationId,
        NotificationDeliveryId,
        NotificationDeliveryChannel,
        NotificationDeliveryStatus,
        NotificationDeliveryAttemptedAt,
        NotificationDeliveryDeliveredAt
    }
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationDelivery {
    id: NotificationDeliveryId,
    notification_id: NotificationId,
    channel: NotificationDeliveryChannel,
    status: NotificationDeliveryStatus,
    attempted_at: Option<NotificationDeliveryAttemptedAt>,
    delivered_at: Option<NotificationDeliveryDeliveredAt>
}

// MARK: Constructors
impl NotificationDelivery {
    pub fn create(
        notification_id: NotificationId,
        channel: NotificationDeliveryChannel
    ) -> Self {
        let id = NotificationDeliveryId::generate();
        let status = NotificationDeliveryStatus::Pending;
        let attempted_at = None;
        let delivered_at = None;

        Self { id, notification_id, channel, status, attempted_at, delivered_at }
    }

    pub fn restore(
        id: NotificationDeliveryId,
        notification_id: NotificationId,
        channel: NotificationDeliveryChannel,
        status: NotificationDeliveryStatus,
        attempted_at: Option<NotificationDeliveryAttemptedAt>,
        delivered_at: Option<NotificationDeliveryDeliveredAt>
    ) -> Result<Self, NotificationDeliveryError> {
        Self::validate_state(status, attempted_at, delivered_at)?;
        Self::validate_delivery_time(attempted_at, delivered_at)?;
        
        Ok(Self { id, notification_id, channel, status, attempted_at, delivered_at })
    }
}

// MARK: Validation
impl NotificationDelivery {
    fn validate_state(
        status: NotificationDeliveryStatus,
        attempted_at: Option<NotificationDeliveryAttemptedAt>,
        delivered_at: Option<NotificationDeliveryDeliveredAt>
    ) -> Result<(), NotificationDeliveryError> {
        match status {
            NotificationDeliveryStatus::Pending => {
                if attempted_at.is_some() || delivered_at.is_some() {
                    return Err(NotificationDeliveryError::InvalidPendingState);
                }
            }

            NotificationDeliveryStatus::Processing => {
                if attempted_at.is_none() || delivered_at.is_some() {
                    return Err(NotificationDeliveryError::InvalidProcessingState);
                }
            }

            NotificationDeliveryStatus::Failed => {
                if attempted_at.is_none() || delivered_at.is_some() {
                    return Err(NotificationDeliveryError::InvalidFailedState);
                }
            }

            NotificationDeliveryStatus::Delivered => {
                if attempted_at.is_none() || delivered_at.is_none() {
                    return Err(NotificationDeliveryError::InvalidDeliveredState);
                }
            }
        }

        Ok(())
    }

    fn validate_delivery_time(
        attempted_at: Option<NotificationDeliveryAttemptedAt>,
        delivered_at: Option<NotificationDeliveryDeliveredAt>
    ) -> Result<(), NotificationDeliveryError> {
        if let (Some(attempted_at), Some(delivered_at)) =
            (attempted_at, delivered_at)
        {
            let attempted_at: NaiveDateTime = attempted_at.into();
            let delivered_at: NaiveDateTime = delivered_at.into();

            if delivered_at < attempted_at {
                return Err(NotificationDeliveryError::InvalidDeliveryTime);
            }
        }

        Ok(())
    }
}

// MARK: Behavior
impl NotificationDelivery {
    pub fn start_processing(
        &mut self,
        attempted_at: NotificationDeliveryAttemptedAt
    ) -> Result<(), NotificationDeliveryError> {
        if self.status != NotificationDeliveryStatus::Pending {
            return Err(NotificationDeliveryError::InvalidStatusTransition);
        }

        self.status = NotificationDeliveryStatus::Processing;
        self.attempted_at = Some(attempted_at);

        Ok(())
    }

    pub fn mark_failed(&mut self) -> Result<(), NotificationDeliveryError> {
        if self.status != NotificationDeliveryStatus::Processing {
            return Err(NotificationDeliveryError::InvalidStatusTransition);
        }

        self.status = NotificationDeliveryStatus::Failed;

        Ok(())
    }

    pub fn mark_delivered(
        &mut self,
        delivered_at: NotificationDeliveryDeliveredAt
    ) -> Result<(), NotificationDeliveryError> {
        if self.status != NotificationDeliveryStatus::Processing {
            return Err(NotificationDeliveryError::InvalidStatusTransition);
        }

        Self::validate_delivery_time(self.attempted_at, Some(delivered_at))?;

        self.status = NotificationDeliveryStatus::Delivered;
        self.delivered_at = Some(delivered_at);

        Ok(())
    }
}

// MARK: Getters
impl NotificationDelivery {
    pub fn id(&self) -> NotificationDeliveryId {
        self.id
    }

    pub fn notification_id(&self) -> NotificationId {
        self.notification_id
    }

    pub fn channel(&self) -> NotificationDeliveryChannel {
        self.channel
    }

    pub fn status(&self) -> NotificationDeliveryStatus {
        self.status
    }

    pub fn attempted_at(&self) -> Option<NotificationDeliveryAttemptedAt> {
        self.attempted_at
    }

    pub fn delivered_at(&self) -> Option<NotificationDeliveryDeliveredAt> {
        self.delivered_at
    }
}
