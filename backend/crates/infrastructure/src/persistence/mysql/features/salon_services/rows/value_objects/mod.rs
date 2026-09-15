use bigdecimal::BigDecimal;
use uuid::Uuid;

crate::mysql_row_value!(MySqlSalonServiceSalonIdRow, Uuid);
crate::mysql_row_value!(MySqlSalonServiceServiceIdRow, Uuid);
crate::mysql_row_value!(MySqlSalonServicePriceRow, BigDecimal);
crate::mysql_row_value!(MySqlSalonServiceIsActiveRow, bool);
