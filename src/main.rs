pub mod configs;
pub mod modules;
use configs::database::{DataBase, DATA_BASE_CONNECTION_INFO};
use modules::routes::RouterServiceCenter;
use sqlx::mysql::MySqlPoolOptions;
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    if let Some(i) = DATA_BASE_CONNECTION_INFO.get(&DataBase::Mysql("xhh_github_index")) {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(i)
            .await?;
        let tt1_rows = sqlx::query("select * from test ").fetch_all(&pool).await;

        for row in tt1_rows {
            println!("{:?}", row);
        }
    }

    if let Some(router_service_center) = RouterServiceCenter::get_single_instance() {
        let app = router_service_center.get_app();
        axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    };

    return Ok(());
}
