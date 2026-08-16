use domain::aggregates::service::value_objects::{ ServiceCategory };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlServiceCategoryRow(String);

impl From<MySqlServiceCategoryRow> for String {
    fn from(row: MySqlServiceCategoryRow) -> Self {
        row.0
    }
}

impl From<String> for MySqlServiceCategoryRow {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl TryFrom<MySqlServiceCategoryRow> for ServiceCategory {
    type Error = anyhow::Error;

    fn try_from(row: MySqlServiceCategoryRow) -> Result<Self, Self::Error> {
        row.0.try_into()
    }
}

impl From<ServiceCategory> for MySqlServiceCategoryRow {
    fn from(entity: ServiceCategory) -> Self {
        Self(entity.into())
    }
}
