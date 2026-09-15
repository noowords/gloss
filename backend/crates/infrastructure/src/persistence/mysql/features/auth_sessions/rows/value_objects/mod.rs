use chrono::NaiveDateTime;
use uuid::Uuid;

crate::mysql_row_value!(MySqlAuthSessionIdRow, Uuid);
crate::mysql_row_value!(MySqlAuthSessionUserIdRow, Uuid);
crate::mysql_row_value!(MySqlAuthSessionRefreshTokenHashRow, Vec<u8>);
crate::mysql_row_value!(MySqlAuthSessionUserAgentRow, String);
crate::mysql_row_value!(MySqlAuthSessionIpAddressRow, Vec<u8>);
crate::mysql_row_value!(MySqlAuthSessionExpiresAtRow, NaiveDateTime);
crate::mysql_row_value!(MySqlAuthSessionLastUsedAtRow, NaiveDateTime);
crate::mysql_row_value!(MySqlAuthSessionRevokedAtRow, NaiveDateTime);
