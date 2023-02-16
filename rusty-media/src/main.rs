use sqlx::{mysql::MySqlPoolOptions, Row};
use tokio;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .connect("mysql://root:devpassword@192.168.0.155/jellyfin")
        .await?;

    let row = sqlx::query!("show TABLES from mysql")
        .fetch_one(&pool)
        .await?;

    dbg!(row);

    Ok(())
}
