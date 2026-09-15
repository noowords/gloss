use domain::aggregates::notifications::notification_delivery::NotificationDelivery;

use super::value_objects::{
    MySqlNotificationDeliveryAttemptedAtRow,
    MySqlNotificationDeliveryChannelRow,
    MySqlNotificationDeliveryDeliveredAtRow,
    MySqlNotificationDeliveryErrorRow,
    MySqlNotificationDeliveryIdRow,
    MySqlNotificationDeliveryNotificationIdRow,
    MySqlNotificationDeliveryReadAtRow,
    MySqlNotificationDeliveryStatusRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlNotificationDeliveryRow {
    pub id: MySqlNotificationDeliveryIdRow,
    pub notification_id: MySqlNotificationDeliveryNotificationIdRow,
    pub channel: MySqlNotificationDeliveryChannelRow,
    pub status: MySqlNotificationDeliveryStatusRow,
    pub attempted_at: Option<MySqlNotificationDeliveryAttemptedAtRow>,
    pub delivered_at: Option<MySqlNotificationDeliveryDeliveredAtRow>,
    pub read_at: Option<MySqlNotificationDeliveryReadAtRow>,
    pub error: Option<MySqlNotificationDeliveryErrorRow>
}

impl TryFrom<MySqlNotificationDeliveryRow> for NotificationDelivery {
    type Error = anyhow::Error;

    fn try_from(row: MySqlNotificationDeliveryRow) -> Result<Self, Self::Error> {
        NotificationDelivery::restore(
            uuid::Uuid::from(row.id).into(),
            uuid::Uuid::from(row.notification_id).into(),
            String::from(row.channel).try_into()?,
            String::from(row.status).try_into()?,
            row.attempted_at.map(|value| chrono::NaiveDateTime::from(value).into()),
            row.delivered_at.map(|value| chrono::NaiveDateTime::from(value).into()),
            row.read_at.map(|value| chrono::NaiveDateTime::from(value).into()),
            row.error.map(|value| String::from(value).try_into()).transpose()?
        )
    }
}

impl From<&NotificationDelivery> for MySqlNotificationDeliveryRow {
    fn from(entity: &NotificationDelivery) -> Self {
        Self {
            id: uuid::Uuid::from(entity.id()).into(),
            notification_id: uuid::Uuid::from(entity.notification_id()).into(),
            channel: String::from(entity.channel()).into(),
            status: String::from(entity.status()).into(),
            attempted_at: entity.attempted_at().map(|value| chrono::NaiveDateTime::from(value).into()),
            delivered_at: entity.delivered_at().map(|value| chrono::NaiveDateTime::from(value).into()),
            read_at: entity.read_at().map(|value| chrono::NaiveDateTime::from(value).into()),
            error: entity.error().map(|value| String::from(value).into())
        }
    }
}
