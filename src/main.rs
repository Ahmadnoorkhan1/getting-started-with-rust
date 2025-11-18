use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

// Import excercise modules
mod sync_async;
use sync_async::backpressure::backpressure_example;
use sync_async::blocking_downloader::blocking_download;
use sync_async::async_downloader::async_download;

// Import load balancer modules
mod client_lb;
use client_lb::load_balancer::{LoadBalancer, Backend};
use client_lb::health_check::{ping, update_health};

// Import caching modules
mod caching;
use caching::cache_aside::{CacheAside, print_metrics};
use caching::singleflight::SingleFlight;


// Axum + Tokio
use axum::{routing::get, Router};
use tokio::net::TcpListener;

// Redis
use redis::aio::ConnectionManager;
use anyhow::Result;

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
                run_server_with_port(3000).await;
            }

            "5" => {
                println!("Running Axum Server with port 3001");
                run_server_with_port(3001).await;
                return;
            }

            "6" => {
                println!("Running Load Balancer Example");
                load_balancer_example_func().await;
                return;
            }

            "7" => {
                println!("Running Cache Aside Example");
                cache_aside_example().await;
                return;
            }

            "8" => {
                println!("Running Single Flight Example");
                single_flight_example().await;
                return;
            }
            _ => {
                println!("Invalid argument. Use: ");
                print_menu();
                return;
            }
        }
    }
}

async fn run_server_with_port(port: u16) {
    let app = Router::new().route("/",get(||async{"ok from rust"}));
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("listening on {}", addr);
    axum::serve(TcpListener::bind(addr).await.unwrap(), app.into_make_service()).await.unwrap();
}

fn print_menu(){
    println!("\n=== BANK CORE TRAINING MENU ===");
    println!("1. Blocking Downloader");
    println!("2. Async Downloader");
    println!("3. Backpressure Example");
    println!("4. Axum Server");
    println!("5. Axum Server (port 3001)");
    println!("6. Load Balancer Example");
    println!("--------------------------------");
}

async fn load_balancer_example_func(){
    let backends = vec![
        Backend {
            url: "http://127.0.0.1:3000".to_string(),
            weight: 3,
            current_connections: Default::default()
        },
        Backend {
            url: "http://127.0.0.1:3001".to_string(),
            weight: 1,
            current_connections: Default::default()
        },
    ];
    
    // ⚠️ MISSING: Check health of all backends FIRST
    println!("\n=== Health Checking Backends ===");
    for backend in &backends {
        let is_alive = ping(&backend.url).await;
        update_health(&backend.url, is_alive);
        println!("{} is {}", backend.url, if is_alive { "ALIVE" } else { "DEAD" });
    }
    println!();
    
    let lb = LoadBalancer::new(backends);
    
    // Round Robin
    if let Some(server)=lb.next_rr().await{
        println!("Round Robin: {}", server.url);
    } else {
        println!("Round Robin: No healthy servers!");
    }

    // Least Connections
    if let Some(server)=lb.next_least_connections().await{
        println!("Least Connections: {}", server.url);
    } else {
        println!("Least Connections: No healthy servers!");
    }

    // Consistent Hashing
    if let Some(server)=lb.next_consistent_hash("test").await{
        println!("Consistent Hashing: {}", server.url);
    } else {
        println!("Consistent Hashing: No healthy servers!");
    }
}

async fn cache_aside_example() -> Result<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let conn = ConnectionManager::new(client).await?;
    let cache = CacheAside::new(conn, 30);
    let acct =cache.get_account("42").await?;
    println!("Account: {:?}", acct);
    print_metrics();
    return Ok(());

}

async fn single_flight_example() -> Result<()>{
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let conn = ConnectionManager::new(client).await?;
    let cache = Arc::new(CacheAside::new(conn, 30));
    let sf = SingleFlight::new(cache);
    let mut tasks = vec![];
    for _ in 0..50 {
        let sf = sf.clone();
        tasks.push(tokio::spawn(async move{
            let r = sf.get("42").await;
            println!("Result: {:?}", r);
        }));
    }
    for t in tasks {
        t.await?;
    }
    print_metrics();
    return Ok(());
}