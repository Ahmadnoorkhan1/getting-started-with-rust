use std::sync::atomic::{AtomicBool, Ordering};
use reqwest::Client;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    static ref HEALTHY: Arc<Mutex<HashMap<String,AtomicBool>>> = Arc::new(Mutex::new(HashMap::new()));
}

// Simple health check ping

pub fn is_healthy(url: &str) -> bool {
    let map = HEALTHY.lock().unwrap();
    if let Some(healthy) = map.get(url) {
        return healthy.load(Ordering::Relaxed);
    }
    true
}

// Update health status
pub fn update_health(url: &str, status: bool) {
    let mut map = HEALTHY.lock().unwrap();
    map.entry(url.to_string()).or_insert_with(|| AtomicBool::new(status)).store(status, Ordering::Relaxed);
}

// Activate ping to check backend

pub async fn ping(url: &str) -> bool {
    Client::new().get(url).timeout(std::time::Duration::from_millis(500)).send().await.map(|r| r.status().is_success()).unwrap_or(false)
}