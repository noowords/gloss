use sqlx::{ MySqlPool };

pub struct MySqlConnection;

impl MySqlConnection {
    pub async fn connect(url: &str) -> Result<MySqlPool, anyhow::Error> {
        Ok(MySqlPool::connect(url).await?)
    }
}
