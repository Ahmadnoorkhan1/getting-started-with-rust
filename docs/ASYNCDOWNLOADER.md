# Async Downloader – Understanding Async/Await and Concurrency in Rust

## What This File Does

This module demonstrates how to download files concurrently using asynchronous Rust. Unlike the blocking example (which uses threads and blocking I/O), this approach uses async/await, futures, and Tokio tasks to handle many downloads efficiently without blocking threads.

## Code Overview

use futures::future::join_all;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::time::Instant;- `join_all`: Waits for all async tasks to complete.
- `Client`: Reqwest async HTTP client.
- `tokio::fs::File` and `AsyncWriteExt`: For async file creation and writing.
- `Instant`: Track total time.

### Function: `async_download`

pub async fn async_download(urls: Vec<&str>) { ... }- Accepts a list of URLs.
- Spawns async tasks to download each file concurrently.
- Saves them as `async_file_0.bin`, `async_file_1.bin`, etc.
- Prints the total duration.

### Flow Breakdown

1. **Start timer**: `let start = Instant::now();`
2. **Create HTTP client**: `let client = Client::new();` and reuse it by cloning.
3. **Create tasks list**: `let mut tasks = vec![];`
4. **Loop and spawn tasks**:
   - Clone the client.
   - Clone the URL string (since it’s moved into the task).
   - Spawn each download with `tokio::spawn(async move { ... })`
   - Download with `client.get(url).send().await?...`
   - Write file with `File::create(...).await` and `write_all(...).await`
5. **Wait for all tasks**: `join_all(tasks).await;`
6. **Print total time**.

## Why Async Instead of Blocking Threads?

- Async uses an event loop (Tokio runtime) instead of spawning OS threads.
- Efficiently handles large numbers of concurrent downloads.
- Fewer resources (threads) used compared to blocking approach.
- Ideal for I/O-bound workloads like network/file I/O.

## Key Concepts

- **Async/Await**: Functions return futures; `await` runs them asynchronously.
- **Futures**: Represent values available later. `join_all` handles many futures.
- **Tokio Tasks**: Spawn lightweight async tasks with `tokio::spawn`.
- **Non-blocking IO**: `tokio::fs` and `reqwest` async client don’t block threads.

## Dependencies Required

Ensure `Cargo.toml` includes:
l
tokio = { version = "1", features = ["rt-multi-thread", "macros", "fs", "io-util"] }
reqwest = { version = "0.12", default-features = true, features = ["blocking"] } # or just default async
futures = "0.3"## Example Usage

#[tokio::main]
async fn main() {
    async_download(vec![
        "https://example.com/img1.jpg",
        "https://example.com/img2.jpg",
        "https://example.com/img3.jpg",
    ]).await;
}## What I Learned

- How to use `tokio::spawn` for async concurrency.
- How to download files using async reqwest client.
- How to write files asynchronously.
- How `join_all` waits for multiple async tasks.
- Difference between blocking threads vs async tasks.

---

This file is a practical example of using async/await and Tokio to efficiently handle multiple network-bound tasks in Rust!