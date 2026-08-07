use domain::aggregates::profile::value_objects::{ ProfileLastName };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlProfileLastNameRow(String);

impl From<MySqlProfileLastNameRow> for ProfileLastName {
    fn from(model: MySqlProfileLastNameRow) -> Self {
        model.0.into()
    }
}

impl From<ProfileLastName> for MySqlProfileLastNameRow {
    fn from(entity: ProfileLastName) -> Self {
        Self(entity.into())
    }
}
