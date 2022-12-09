use axum::{routing::get, Router};
use lazy_static::lazy_static;
use tokio::sync::Mutex;

/* lazy_static! {
    pub static ref Router_Service_Center: Mutex<Router> =
        { Mutex::new(Router::new().route('/', get(test))) };
} */
