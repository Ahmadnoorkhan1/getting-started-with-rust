use futures::future::join_all;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::time::Instant;

pub async fn async_download(urls: Vec<&str>) {
    let start = Instant::now();
    let client = Client::new();
    let mut tasks = vec![];
    for (i, url) in urls.iter().enumerate(){
        let client = client.clone();
        let url = url.to_string();
        tasks.push(tokio::spawn(async move {
            let bytes = client.get(url).send().await.unwrap().bytes().await.unwrap();
            let file_name = format!("async_file_{}.bin", i);
            let mut file = File::create(file_name).await.unwrap();
            file.write_all(&bytes).await.unwrap();
        }));
    }
    join_all(tasks).await;
    println!("Async download finished in {:?}", start.elapsed());
}