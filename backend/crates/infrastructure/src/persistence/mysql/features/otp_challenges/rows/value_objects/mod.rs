use chrono::NaiveDateTime;
use uuid::Uuid;

crate::mysql_row_value!(MySqlOtpChallengeIdRow, Uuid);
crate::mysql_row_value!(MySqlOtpChallengeProviderRow, String);
crate::mysql_row_value!(MySqlOtpChallengeSubjectRow, String);
crate::mysql_row_value!(MySqlOtpChallengePurposeRow, String);
crate::mysql_row_value!(MySqlOtpChallengeCodeHashRow, Vec<u8>);
crate::mysql_row_value!(MySqlOtpChallengeAttemptsRow, u16);
crate::mysql_row_value!(MySqlOtpChallengeExpiresAtRow, NaiveDateTime);
crate::mysql_row_value!(MySqlOtpChallengeVerifiedAtRow, NaiveDateTime);
crate::mysql_row_value!(MySqlOtpChallengeConsumedAtRow, NaiveDateTime);
