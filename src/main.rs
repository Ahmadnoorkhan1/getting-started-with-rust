use std::env;
use std::net::SocketAddr;

// Import excercise modules
mod sync_async;
use sync_async::backpressure::backpressure_example;
use sync_async::blocking_downloader::blocking_download;
use sync_async::async_downloader::async_download;

// Axum + Tokio
use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Read CLI Arguments
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str(){
            "1" => {
                println!("Running Blocking Downloader");
                let urls = vec![
                    "https://www.rust-lang.org/static/images/logo-200x200.png",
                    "https://www.rust-lang.org/static/images/logo-200x200.png",
                    "https://www.rust-lang.org/static/images/logo-200x200.png",
                ];
                blocking_download(urls);
                return;
            }

            "2" => {
                println!("Running Async Downloader");
                let urls = vec![
                    "https://www.rust-lang.org/static/images/logo-200x200.png",
                    "https://www.rust-lang.org/static/images/logo-200x200.png",
                    "https://www.rust-lang.org/static/images/logo-200x200.png",
                ];
                async_download(urls).await;
                return;
            }

            "3" => {
                println!("Running Backpressure Example");
                backpressure_example().await;
                return;
            }

            "4" => {
                println!("Running Axum Server");
                run_server().await;
            }

            _ => {
                println!("Invalid argument. Use: ");
                print_menu();
                return;
            }
        }
    }
}

async fn run_server() {
    let app = Router::new().route("/",get(||async{"ok from rust"}));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::serve(TcpListener::bind(addr).await.unwrap(), app.into_make_service()).await.unwrap();
}

async fn print_menu(){
    println!("\n=== BANK CORE TRAINING MENU ===");
    println!("1. Blocking Downloader");
    println!("2. Async Downloader");
    println!("3. Backpressure Example");
    println!("4. Axum Server");
    println!("--------------------------------");
}
