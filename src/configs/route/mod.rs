use crate::hooks::services::router::RouterServiceCenter;
use axum::{routing::get, Router};

async fn index() {
    println!("21312");
}
pub fn routes() {
    if let Some(i) = RouterServiceCenter::get_single_instance() {
        let route = Router::new().route("/", get(index));
        i.insert(route)
    }
}
