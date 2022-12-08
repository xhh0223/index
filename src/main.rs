pub mod configs;
pub mod modules;

use configs::route::ROUTER_SERVICE_CENTER;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let router = ROUTER_SERVICE_CENTER.lock();
    
    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
    /*     if let Some(i) = DATA_BASE_CONNECTION_INFO.get(&DataBase::Mysql("xhh_github_index")) {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(i)
            .await?;
        let tt1_rows = sqlx::query("select * from test ").fetch_all(&pool).await;

        for row in tt1_rows {
            println!("{:?}", row);
        }
    } */

    return Ok(());
}
