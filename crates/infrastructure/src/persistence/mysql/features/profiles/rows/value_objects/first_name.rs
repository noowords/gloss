use domain::aggregates::profile::value_objects::{ ProfileFirstName };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlProfileFirstNameRow(String);

impl From<MySqlProfileFirstNameRow> for ProfileFirstName {
    fn from(model: MySqlProfileFirstNameRow) -> Self {
        model.0.into()
    }
}

impl From<ProfileFirstName> for MySqlProfileFirstNameRow {
    fn from(entity: ProfileFirstName) -> Self {
        Self(entity.into())
    }
}
