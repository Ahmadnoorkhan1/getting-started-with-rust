use std::sync:: {Arc, Mutex};
use crate::client_lb::health_check::is_healthy;

#[derive(Clone)]

pub struct Backend {
    pub url:String,
    pub weight:u32,
    pub current_connections: Arc<Mutex<u32>>,
}


#[derive(Clone)]

pub struct LoadBalancer {
    pub backends: Vec<Backend>,
    rr_index: Arc<Mutex<usize>>,
}

impl LoadBalancer {
    pub fn new(backends: Vec<Backend>) -> Self {
        LoadBalancer {
            backends,
            rr_index: Arc::new(Mutex::new(0))
        }
    }

    // Weighted Round Robin
    pub async fn next_rr(&self) -> Option<Backend> {
        let mut idx = self.rr_index.lock().unwrap();
        for _ in 0..self.backends.len() {
            let backend = &self.backends[*idx];
            *idx = (*idx + 1) % self.backends.len();
            if is_healthy(&backend.url) {
                return Some(backend.clone());
            }
        }
        None
    }

    // Least Connections
    pub async fn next_least_connections(&self) -> Option<Backend> {
        self.backends.iter().filter(|b| is_healthy(&b.url)).min_by_key(|b| *b.current_connections.lock().unwrap()).cloned()
    }

    // Consistent Hashting (simple modulo for demonstration)
    pub async fn next_consistent_hash(&self, key:&str) -> Option<Backend> {
        let healthy: Vec<_> = self.backends.iter().filter(|b| is_healthy(&b.url)).collect();
        if healthy.is_empty() {
            return None;
        }
        let hash = crc32fast::hash(key.as_bytes()) as usize;
        let idx = hash % healthy.len();
        Some(healthy[idx].clone())
    }
}
