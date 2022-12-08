use axum::{routing::get, Router};
use lazy_static::lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref ROUTER_SERVICE_CENTER: Mutex<Router> =
        { Mutex::new(Router::new().route('/', get(test))) };
}

async fn test() -> &'static str {}
