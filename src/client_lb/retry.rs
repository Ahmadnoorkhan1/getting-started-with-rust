use std::time::Duration;
use tokio::time::sleep;

pub async fn retry_async<F, Fut, T>(mut f:F, retries:u32, delay_ms:u64) -> Option<T> 
where 
F: FnMut() -> Fut,
Fut: std::future::Future<Output = Option<T>>,
{
    for _ in 0..retries{
    if let Some(result) = f().await {
        return Some(result);
    }
    sleep(Duration::from_millis(delay_ms)).await;
    }
    None
}