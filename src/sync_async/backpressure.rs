use tokio::sync::mpsc;
use tokio::time::{sleep,Duration};

pub async fn backpressure_example() {
    let (tx,mut rx) = mpsc::channel::<i32>(5);

    tokio::spawn( async move{
        for i in 0..20{
            if tx.send(i).await.is_err(){
                println!("receiver dropped");
                return;
            }
            println!("Produced {}",i);
        }
    });
    while let Some(value) = rx.recv().await{
        println!("Consumed {}",value);
        sleep(Duration::from_millis(500)).await;
    }
} 