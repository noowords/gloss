use domain::aggregates::services::service_addon_rule::{ ServiceAddonRule };

use super::value_objects::{ MySqlServiceAddonRuleAddonServiceIdRow, MySqlServiceAddonRulePrimaryServiceIdRow };

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlServiceAddonRuleRow {
    pub primary_service_id: MySqlServiceAddonRulePrimaryServiceIdRow,
    pub addon_service_id: MySqlServiceAddonRuleAddonServiceIdRow
}

impl TryFrom<MySqlServiceAddonRuleRow> for ServiceAddonRule {
    type Error = anyhow::Error;

    fn try_from(row: MySqlServiceAddonRuleRow) -> Result<Self, Self::Error> {
        ServiceAddonRule::restore(uuid::Uuid::from(row.primary_service_id).into(), uuid::Uuid::from(row.addon_service_id).into())
    }
}

impl From<&ServiceAddonRule> for MySqlServiceAddonRuleRow {
    fn from(entity: &ServiceAddonRule) -> Self {
        Self {
            primary_service_id: uuid::Uuid::from(entity.primary_service_id()).into(),
            addon_service_id: uuid::Uuid::from(entity.addon_service_id()).into()
        }
    }
}
