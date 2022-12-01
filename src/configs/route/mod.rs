pub mod index;
use crate::hooks::services::router::RouterServiceCenter;
use axum::Router;
pub fn routes(){
  if let Some(i) = RouterServiceCenter::get_single_instance(){
    Router::new().route("/", ||{
      
    });
    i.insert(route("/", get(|| async { "Hello, World!" })))
  }
}