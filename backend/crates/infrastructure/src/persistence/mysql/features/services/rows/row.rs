use domain::aggregates::services::service::{ Service };

use super::value_objects::{
    MySqlServiceIdRow,
    MySqlServiceNameRow,
    MySqlServiceDescriptionRow,
    MySqlServicePreviewUrlRow,
    MySqlServiceCategoryRow,
    MySqlServiceKindRow,
    MySqlServiceDurationMinutesRow,
    MySqlServiceIsActiveRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlServiceRow {
    pub id: MySqlServiceIdRow,
    pub name: MySqlServiceNameRow,
    pub description: Option<MySqlServiceDescriptionRow>,
    pub preview_url: Option<MySqlServicePreviewUrlRow>,
    pub category: MySqlServiceCategoryRow,
    pub kind: MySqlServiceKindRow,
    pub duration_minutes: MySqlServiceDurationMinutesRow,
    pub is_active: MySqlServiceIsActiveRow
}

impl TryFrom<MySqlServiceRow> for Service {
    type Error = anyhow::Error;
    
    fn try_from(row: MySqlServiceRow) -> Result<Self, Self::Error> {
        Self::restore(
            row.id.into(),
            row.name.into(),
            row.description.map(|d| d.into()),
            row.preview_url.map(|url| url.into()),
            row.category.try_into()?,
            row.kind.try_into()?,
            row.duration_minutes.try_into()?,
            row.is_active.into()
        )
    }
}

impl From<&Service> for MySqlServiceRow {
    fn from(entity: &Service) -> Self {
        Self {
            id: entity.id().into(),
            name: entity.name().into(),
            description: entity.description().map(|d| d.into()),
            preview_url: entity.preview_url().map(|url| url.into()),
            category: entity.category().into(),
            kind: entity.kind().into(),
            duration_minutes: entity.duration_minutes().into(),
            is_active: entity.is_active().into()
        }
    }
}
