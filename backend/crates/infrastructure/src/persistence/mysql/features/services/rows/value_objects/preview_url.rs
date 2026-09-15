use domain::aggregates::services::service::value_objects::{ ServicePreviewUrl };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlServicePreviewUrlRow(String);

impl From<MySqlServicePreviewUrlRow> for String {
    fn from(row: MySqlServicePreviewUrlRow) -> Self {
        row.0
    }
}

impl From<String> for MySqlServicePreviewUrlRow {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<MySqlServicePreviewUrlRow> for ServicePreviewUrl {
    fn from(row: MySqlServicePreviewUrlRow) -> Self {
        row.0.try_into().expect("Invalid service preview url")
    }
}

impl From<ServicePreviewUrl> for MySqlServicePreviewUrlRow {
    fn from(entity: ServicePreviewUrl) -> Self {
        Self(entity.into())
    }
}
