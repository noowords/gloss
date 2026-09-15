use uuid::Uuid;

crate::mysql_row_value!(MySqlSalonIdRow, Uuid);
crate::mysql_row_value!(MySqlSalonCodeRow, String);
crate::mysql_row_value!(MySqlSalonNameRow, String);
crate::mysql_row_value!(MySqlSalonCityRow, String);
crate::mysql_row_value!(MySqlSalonAddressRow, String);
crate::mysql_row_value!(MySqlSalonTimezoneRow, String);
crate::mysql_row_value!(MySqlSalonStatusRow, String);
