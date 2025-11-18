use anyhow :: Result;
use redis :: {aio::ConnectionManager, AsyncCommands};
use serde :: {Deserialize, Serialize};
use std::sync::atomic:: {AtomicU64, Ordering};
use tokio::time::{sleep, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: u64,
    pub balance_cents: i64,
    pub name: String,
}

// ---------------------- Metrics ----------------------

pub static CACHE_HITS: AtomicU64 = AtomicU64::new(0);
pub static CACHE_MISSES: AtomicU64 = AtomicU64::new(0);
pub static DB_FETCHES: AtomicU64 = AtomicU64::new(0);

pub fn print_metrics(){
    println!("[metrics] hits = {} misses = {} db_fetches = {}", CACHE_HITS.load(Ordering::Relaxed), CACHE_MISSES.load(Ordering::Relaxed), DB_FETCHES.load(Ordering::Relaxed));
}

// ---------------------- Cache-aside Layer ----------------------

pub struct CacheAside{
    conn: ConnectionManager,
    ttl: usize,
}

impl CacheAside{
    pub fn new (conn:ConnectionManager, ttl:usize) -> Self{
        Self { conn,ttl }
    }

    pub async fn get_account(&self, id:&str) -> Result<Account>{
        let key = format!("acct:{}", id);
        
        // Try redis first

        let mut c  = self.conn.clone();
        if let Ok(Some(json)) = c.get::<_,Option<String>>(&key).await{
            CACHE_HITS.fetch_add(1,Ordering::Relaxed);
            return Ok(serde_json::from_str(&json)?);
        }

        CACHE_MISSES.fetch_add(1,Ordering::Relaxed);

        // Fetch from db (simulated)
        let acct = self.fetch_from_db(id).await?;
        
        // Store in redis
        let json = serde_json::to_string(&acct)?;
        let _: () = c.set_ex(key, json, self.ttl as u64).await?;

        Ok(acct)
    }

    async fn fetch_from_db(&self, id:&str) -> Result<Account>{
        DB_FETCHES.fetch_add(1,Ordering::Relaxed);

        // Simulate latency
        sleep(Duration::from_millis(150)).await;

        Ok(Account {
            id: id.parse::<u64>()?,
            balance_cents: 1000,
            name: "John Doe".to_string(),
        })
    }

}


