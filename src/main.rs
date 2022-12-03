pub mod modules;
use modules::routes::RouterServiceCenter;
#[tokio::main]
async fn main() {
    if let Some(router_service_center) = RouterServiceCenter::get_single_instance() {
        let app = router_service_center.get_app();
        axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
