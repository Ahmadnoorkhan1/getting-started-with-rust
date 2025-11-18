mod sync_async;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
#[tokio::main]
async fn main() {
    use sync_async::blocking_downloader::blocking_download;
    let urls = vec![
        "https://www.rust-lang.org/static/images/logo-200x200.png",
        "https://www.rust-lang.org/static/images/logo-200x200.png",
        "https://www.rust-lang.org/static/images/logo-200x200.png",
    ];
    blocking_download(urls); 
    println!("Blocking download finished");   
    let app = Router::new().route("/",get(||async{"ok from rust"}));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::serve(TcpListener::bind(addr).await.unwrap(), app.into_make_service()).await.unwrap();
}
