A Rust-based HTTP web server built with Axum and Tokio.

## Overview

This is a simple HTTP server that listens on `localhost:3000` and responds to GET requests at the root path (`/`) with a simple "ok from rust" message.

## Project Structure

- **`Cargo.toml`** - Project manifest and dependency configuration
- **`src/main.rs`** - Main application entry point with HTTP server setup

## Learning Resources

- [Blocking Downloader](./docs/BLOCKINGDOWNLOADER.md) - How I learned about synchronous functions and file downloads using URLs

## Dependencies (`Cargo.toml`)

### Tokio (`tokio`)
- **Purpose**: Asynchronous runtime for Rust
- **Features enabled**:
  - `rt-multi-thread`: Enables multi-threaded async runtime for parallel task execution
  - `macros`: Provides `#[tokio::main]` macro for async main function
- **Why**: Axum requires an async runtime to handle concurrent HTTP requests efficiently

### Axum (`axum`)
- **Purpose**: Modern, ergonomic web framework built on top of Tokio and Tower
- **Version**: 0.7
- **Why**: Provides HTTP routing, request handling, and response building functionality

## Code Explanation (`src/main.rs`)

### Line 1: Import Router and GET handler
use axum::{routing::get, Router};- Imports the `Router` type for defining routes
- Imports the `get` function for handling GET requests

### Line 2: Import SocketAddr
use std::net::SocketAddr;- Used to specify the IP address and port for the server

### Line 3: Import TcpListener
use tokio::net::TcpListener;- Asynchronous TCP listener for accepting incoming connections

### Line 4: Tokio main macro
#[tokio::main]- Transforms the `main()` function into an async entry point that initializes the Tokio runtime

### Line 6: Create router and define routeust
let app = Router::new().route("/", get(|| async { "ok from rust" }));- Creates a new router instance
- Adds a route for the root path (`/`) that handles GET requests
- Returns the string `"ok from rust"` as the response

### Line 7: Define server address
let addr = SocketAddr::from(([127, 0, 0, 1], 3000));- Sets the server to listen on `127.0.0.1` (localhost) port `3000`

### Line 9: Start the server
axum::serve(TcpListener::bind(addr).await.unwrap(), app).await.unwrap();- Binds a TCP listener to the specified address
- Starts the Axum server with the router
- `.await.unwrap()` waits for the server to start and handles errors (crashes on failure)

## How to Run

1. Make sure you have Rust installed (`cargo` should be available)

2. Run the server:
 
   cargo run
   3. The server will start and print:
   