use crate::caching::cache_aside::CacheAside;
use crate::caching::cache_aside::Account;

use anyhow::Result;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex,Notify};

#[derive(Clone)]

pub struct SingleFlight{
    map: Arc<Mutex<HashMap<String,Arc<Notify>>>>,
    cache: Arc<CacheAside>,
}

impl SingleFlight {
    pub fn new (cache: Arc<CacheAside>) -> Self{
        Self{
            map: Arc::new(Mutex::new(HashMap::new())),
            cache,
        }
    }
    pub async fn get(&self, key: &str) -> Result<Account>{
        // Try cache first
        if let Ok(val) = self.cache.get_account(key).await{
            return Ok(val);
        }

        let wait = {
            let mut map = self.map.lock().await;

            // Somone else fethcing
            if let Some(w) = map.get(key){
                Some(w.clone())
            }else {
                let n = Arc::new(Notify::new());
                map.insert(key.to_string(), n.clone());
                None
            }
        };

        if let Some(w) = wait{
            w.notified().await;
            return self.cache.get_account(key).await;
        }

        // Leader Path - fetch From DB + populate Redis
        let res  = self.cache.get_account(key).await;

        let mut map = self.map.lock().await;

        if let Some(n) = map.remove(key){
            n.notify_waiters();
        }

        res
    }
}