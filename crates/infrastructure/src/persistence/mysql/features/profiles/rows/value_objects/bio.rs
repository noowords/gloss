use domain::aggregates::profile::value_objects::{ ProfileBio };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlProfileBioRow(String);

impl From<MySqlProfileBioRow> for ProfileBio {
    fn from(model: MySqlProfileBioRow) -> Self {
        model.0.into()
    }
}

impl From<ProfileBio> for MySqlProfileBioRow {
    fn from(entity: ProfileBio) -> Self {
        Self(entity.into())
    }
}
