pub mod configs;
pub mod modules;

use std::error::Error;

use configs::database::GITHUB_INDEX_MYSQL;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let v = GITHUB_INDEX_MYSQL.getPool(2).await?;
    sqlx::query("select * from image").fetch_all(&v).await?;
    Ok(())
}
