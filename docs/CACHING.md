
---

## Key Concepts I Learned

### 1. **Cache-Aside Pattern**
- Application code manages cache (not automatic)
- Check cache → if miss → fetch from DB → store in cache
- Simple and flexible
- Used in: Rails, Django, most web frameworks

### 2. **Single-Flight Pattern**
- Deduplicate concurrent requests for same key
- Leader does the work, followers wait and share result
- Prevents cache stampede / thundering herd
- Used in: Go's `singleflight`, Facebook, Google

### 3. **Redis Operations**
- `.get()` - Get value (returns `Option<String>`)
- `.set_ex()` - Set with expiration (TTL)
- `ConnectionManager::clone()` - Clone connection for async operations
- Serialize to JSON before storing (Redis stores strings)

### 4. **Thread Safety**
- `AtomicU64` - Fast, lock-free counters
- `Arc<Mutex<HashMap>>` - Shared HashMap with locks
- `Arc<Notify>` - Notification mechanism shared across threads
- `Mutex::lock().await` - Async lock (doesn't block thread)

### 5. **Async Patterns**
- `.await` - Wait for async operations
- Clone connections (`self.conn.clone()`) for concurrent use
- `Notify::notified().await` - Wait for notification
- `Notify::notify_waiters()` - Wake up all waiters

### 6. **Performance Metrics**
- Track hits/misses to measure cache effectiveness
- `AtomicU64::fetch_add()` - Thread-safe increment
- `Ordering::Relaxed` - Fastest atomic ordering (no synchronization needed)

---

## Real-World Applications

### Cache-Aside Pattern:
- **Database queries** - Cache expensive SELECT statements
- **API responses** - Cache third-party API calls
- **Computed values** - Cache heavy calculations

### Single-Flight Pattern:
- **Cache warming** - Prevent duplicate cache population
- **API rate limiting** - Multiple requests = one external API call
- **Database queries** - Prevent duplicate queries during traffic spikes
- **Thundering herd** - Prevents 1000 requests hitting DB at once

---

## What This Teaches

This is **production-grade caching**:
- ✅ Prevents expensive operations
- ✅ Handles concurrent requests efficiently
- ✅ Thread-safe (multiple requests can run simultaneously)
- ✅ Metrics tracking (monitor cache performance)
- ✅ Patterns used by Facebook, Google, Netflix

**These are the same patterns used in:**
- Go's `singleflight` package
- Rails fragment caching
- Django caching framework
- Facebook's Memcached
- Redis best practices
