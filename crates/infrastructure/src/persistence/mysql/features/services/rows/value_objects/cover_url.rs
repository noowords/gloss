use domain::aggregates::service::value_objects::{ ServiceCoverUrl };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlServiceCoverUrlRow(String);

impl From<MySqlServiceCoverUrlRow> for String {
    fn from(row: MySqlServiceCoverUrlRow) -> Self {
        row.0
    }
}

impl From<String> for MySqlServiceCoverUrlRow {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<MySqlServiceCoverUrlRow> for ServiceCoverUrl {
    fn from(row: MySqlServiceCoverUrlRow) -> Self {
        row.0.into()
    }
}

impl From<ServiceCoverUrl> for MySqlServiceCoverUrlRow {
    fn from(entity: ServiceCoverUrl) -> Self {
        Self(entity.into())
    }
}
