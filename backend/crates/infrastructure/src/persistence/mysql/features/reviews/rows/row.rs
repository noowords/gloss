use domain::aggregates::reviews::review::{ Review };

use super::value_objects::{
    MySqlReviewAppointmentIdRow,
    MySqlReviewCommentRow,
    MySqlReviewIdRow,
    MySqlReviewRatingRow,
    MySqlReviewStatusRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlReviewRow {
    pub id: MySqlReviewIdRow,
    pub appointment_id: MySqlReviewAppointmentIdRow,
    pub rating: MySqlReviewRatingRow,
    pub comment: Option<MySqlReviewCommentRow>,
    pub status: MySqlReviewStatusRow
}

impl TryFrom<MySqlReviewRow> for Review {
    type Error = anyhow::Error;

    fn try_from(row: MySqlReviewRow) -> Result<Self, Self::Error> {
        Review::restore(
            uuid::Uuid::from(row.id).into(),
            uuid::Uuid::from(row.appointment_id).into(),
            u8::from(row.rating).try_into()?,
            row.comment.map(|value| String::from(value).try_into()).transpose()?,
            String::from(row.status).try_into()?
        )
    }
}

impl From<&Review> for MySqlReviewRow {
    fn from(entity: &Review) -> Self {
        Self {
            id: uuid::Uuid::from(entity.id()).into(),
            appointment_id: uuid::Uuid::from(entity.appointment_id()).into(),
            rating: u8::from(entity.rating()).into(),
            comment: entity.comment().map(|value| String::from(value).into()),
            status: String::from(entity.status()).into()
        }
    }
}
