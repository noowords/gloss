use domain::aggregates::notifications::notification::Notification;

use super::value_objects::{
    MySqlNotificationIdRow,
    MySqlNotificationMessageRow,
    MySqlNotificationTitleRow,
    MySqlNotificationTypeRow,
    MySqlNotificationUserIdRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlNotificationRow {
    pub id: MySqlNotificationIdRow,
    pub user_id: MySqlNotificationUserIdRow,
    pub r#type: MySqlNotificationTypeRow,
    pub title: MySqlNotificationTitleRow,
    pub message: MySqlNotificationMessageRow
}

impl TryFrom<MySqlNotificationRow> for Notification {
    type Error = anyhow::Error;

    fn try_from(row: MySqlNotificationRow) -> Result<Self, Self::Error> {
        Notification::restore(
            uuid::Uuid::from(row.id).into(),
            uuid::Uuid::from(row.user_id).into(),
            String::from(row.r#type).try_into()?,
            String::from(row.title).try_into()?,
            String::from(row.message).try_into()?
        )
    }
}

impl From<&Notification> for MySqlNotificationRow {
    fn from(entity: &Notification) -> Self {
        Self {
            id: uuid::Uuid::from(entity.id()).into(),
            user_id: uuid::Uuid::from(entity.user_id()).into(),
            r#type: String::from(entity.r#type()).into(),
            title: String::from(entity.title()).into(),
            message: String::from(entity.message()).into()
        }
    }
}
