use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
#[tokio::main]
async fn main() {
    let app = Router::new().route("/",get(||async{"ok from rust"}));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::serve(TcpListener::bind(addr).await.unwrap(), app.into_make_service()).await.unwrap();
}
