# Load Balancing - What I Learned

## What This Is

This module implements a **load balancer** that distributes requests across multiple backend servers. It consists of three files that work together:
- `load_balancer.rs` - Decides which server to use
- `health_check.rs` - Checks if servers are alive
- `retry.rs` - Retries failed operations

## File 1: `load_balancer.rs` - Load Balancing Algorithms

### What It Does

Decides **which backend server** to send each request to using different algorithms.

### Data Structures

**`Backend` struct (Lines 6-10):**ust
pub struct Backend {
    pub url: String,
    pub weight: u32,
    pub current_connections: Arc<Mutex<u32>>,
}**What I learned:**
- `Arc<Mutex<u32>>` - Thread-safe counter
  - `Arc` - Shared across threads
  - `Mutex` - Only one thread can modify at a time
  - `u32` - Tracks connection count
- Needed to track how many connections each server has

**`LoadBalancer` struct (Lines 15-18):**ust
pub struct LoadBalancer {
    pub backends: Vec<Backend>,
    rr_index: Arc<Mutex<usize>>,
}**What I learned:**
- `rr_index` stores the current round-robin position
- Must be thread-safe (multiple requests can call `next_rr()` at same time)

### Algorithm 1: Round Robin (`next_rr`)

**What it does:** Cycles through servers: A → B → C → A → B...

**Code (Lines 29-39):**
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
}**What I learned:**
- `*idx` - Dereference to get the `usize` value
- `(*idx + 1) % self.backends.len()` - Increment and wrap around
- Loop skips unhealthy servers automatically
- Returns `None` if no healthy servers

### Algorithm 2: Least Connections (`next_least_connections`)

**What it does:** Picks the server with fewest active connections.

**Code (Lines 42-44):**
pub async fn next_least_connections(&self) -> Option<Backend> {
    self.backends.iter()
        .filter(|b| is_healthy(&b.url))
        .min_by_key(|b| *b.current_connections.lock().unwrap())
        .cloned()
}**What I learned:**
- `.filter()` - Only keep healthy servers
- `.min_by_key()` - Find minimum by connection count
- `.lock().unwrap()` - Thread-safe access to counter
- `.cloned()` - Clone the backend (iterator returns references)

### Algorithm 3: Consistent Hashing (`next_consistent_hash`)

**What it does:** Same key always maps to same server (useful for caching).

**Code (Lines 47-55):**
pub async fn next_consistent_hash(&self, key: &str) -> Option<Backend> {
    let healthy: Vec<_> = self.backends.iter()
        .filter(|b| is_healthy(&b.url))
        .collect();
    let hash = crc32fast::hash(key.as_bytes()) as usize;
    let idx = hash % healthy.len();
    Some(healthy[idx].clone())
}**What I learned:**
- `crc32fast::hash()` - Fast hash function
- `key.as_bytes()` - Convert string to bytes for hashing
- `hash % healthy.len()` - Map hash to valid array index
- Same key always produces same hash → same server

---

## File 2: `health_check.rs` - Health Monitoring

### What It Does

Checks if backend servers are alive and responding to requests.

### Global State (Lines 6-8)

lazy_static::lazy_static! {
    static ref HEALTHY: Arc<Mutex<HashMap<String,AtomicBool>>> = ...
}**What I learned:**
- `lazy_static!` - Initializes static variable on first access
- `Arc<Mutex<HashMap<...>>>` - Thread-safe HashMap
- Key: URL string
- Value: `AtomicBool` - Thread-safe boolean (fast reads without lock)

**Why this design:**
- Shared across all threads
- Fast reads (`AtomicBool` doesn't need lock)
- Thread-safe writes (Mutex protects HashMap)

### `is_healthy` Function (Lines 12-18)

pub fn is_healthy(url: &str) -> bool {
    let map = HEALTHY.lock().unwrap();
    if let Some(healthy) = map.get(url) {
        return healthy.load(Ordering::Relaxed);
    }
    true  // Default: assume healthy
}**What I learned:**
- `.lock()` - Acquire Mutex lock
- `.get(url)` - Lookup in HashMap
- `.load(Ordering::Relaxed)` - Atomic read (no lock needed for this)
- Returns `true` by default if URL not found (optimistic)

### `update_health` Function (Lines 21-24)

pub fn update_health(url: &str, status: bool) {
    let mut map = HEALTHY.lock().unwrap();
    map.entry(url.to_string())
        .or_insert_with(|| AtomicBool::new(status))
        .store(status, Ordering::Relaxed);
}**What I learned:**
- `.entry()` - Get or create HashMap entry
- `.or_insert_with()` - Create if missing
- `.store()` - Atomic write (thread-safe)
- One-liner for thread-safe insert/update

### `ping` Function (Lines 28-30)

pub async fn ping(url: &str) -> bool {
    Client::new().get(url)
        .timeout(std::time::Duration::from_millis(500))
        .send().await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}**What I learned:**
- Async HTTP client (`reqwest::Client`, not `blocking::Client`)
- `.timeout()` - Set max wait time (500ms)
- `.send().await` - Send request and wait (async)
- `.map()` - Transform Result<Response, Error> to Option<bool>
- Returns `false` on timeout or error

**Critical lesson:** Can't use blocking client in async function - causes panic!

---

## File 3: `retry.rs` - Retry Logic

### What It Does

Retries failed operations multiple times with delays between attempts.

### Function (Lines 4-16)
t
pub async fn retry_async<F, Fut, T>(
    mut f: F,
    retries: u32,
    delay_ms: u64
) -> Option<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Option<T>>,
{
    for _ in 0..retries {
        if let Some(result) = f().await {
            return Some(result);
        }
        sleep(Duration::from_millis(delay_ms)).await;
    }
    None
}**What I learned:**
- Generic function: `<F, Fut, T>`
- `F: FnMut() -> Fut` - Function that returns a Future
- `Fut: Future<Output = Option<T>>` - Future that returns Option
- Loop tries up to `retries` times
- `Some(result)` = success, return immediately
- `None` = failure, wait and retry
- Returns `None` if all retries fail

**How it works:**
1. Call function `f()`
2. If success (`Some`) → return immediately
3. If failure (`None`) → wait `delay_ms` milliseconds
4. Retry until success or out of retries

---

## How They Work Together

### Complete Flow (from main.rs)

// 1. Define backends
let backends = vec![Backend { url: "...", ... }];

// 2. Health check
for backend in &backends {
    let is_alive = ping(&backend.url).await;  // Ping server
    update_health(&backend.url, is_alive);    // Store result
}

// 3. Create load balancer
let lb = LoadBalancer::new(backends);

// 4. Select backend
let server = lb.next_rr().await;  // Uses is_healthy() internally**Flow:**
1. **Health check** - Ping all servers, update status
2. **Load balancer** - Check `is_healthy()` (fast lookup)
3. **Select algorithm** - Pick server based on algorithm
4. **Retry** - If request fails, retry with `retry_async()`

---

## Key Concepts I Learned

### 1. **Thread Safety**
- `Arc<T>` - Share data across threads
- `Mutex<T>` - Exclusive access (one thread at a time)
- `AtomicBool` - Fast, lock-free boolean reads/writes
- Needed because multiple requests run concurrently

### 2. **Async vs Blocking**
- `reqwest::blocking::Client` - Blocks thread (can't use in async)
- `reqwest::Client` - Non-blocking (use in async)
- `.await` - Wait for async operations
- Critical: Can't mix blocking and async!

### 3. **Load Balancing Algorithms**
- **Round Robin** - Simple rotation (A→B→C→A)
- **Least Connections** - Pick least busy server
- **Consistent Hashing** - Same key → same server (for caching)

### 4. **Health Checking**
- Background health checks keep status updated
- Fast in-memory lookup (`AtomicBool`)
- Skips unhealthy servers automatically

### 5. **Retry Logic**
- Handle transient failures
- Configurable retries and delays
- Generic function (works with any async operation)

---

## What This Teaches

This is **system engineering** - not just CRUD:
- Distributed systems architecture
- Failure handling and resilience
- Thread-safe concurrent programming
- Production patterns used by Netflix, AWS, Google Cloud
