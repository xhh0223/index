use axum::routing::get;
use axum::Router;

use super::RouterServiceCenter;

pub fn register() {
    match RouterServiceCenter::get_single_instance() {
        Some(router_service_center) => {
            router_service_center.register_router(Router::new().route("/", get(index)));
        }
        _ => {}
    }
}

async fn index() -> &'static str {
    "312321"
}
