use uuid::Uuid;

crate::mysql_row_value!(MySqlReviewIdRow, Uuid);
crate::mysql_row_value!(MySqlReviewAppointmentIdRow, Uuid);
crate::mysql_row_value!(MySqlReviewRatingRow, u8);
crate::mysql_row_value!(MySqlReviewCommentRow, String);
crate::mysql_row_value!(MySqlReviewStatusRow, String);
