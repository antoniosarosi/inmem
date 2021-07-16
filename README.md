# Inmem

Dead simple and inefficient in-memory database.

### Running

```bash
cargo run -- 8080
```

### Connecting

```bash
# Use any TCP client
netcat localhost 8080
```

### Commands

```bash
SET lang Rust
GET lang # Rust
DEL lang
SET "Some long key" "Some long value"
```
