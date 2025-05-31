# gfs-rust

A Rust-based implementation of the [Google File System (GFS)](https://research.google/pubs/pub51/) architecture.  
This project is designed to explore distributed systems, concurrency, and asynchronous programming in Rust using a modular, test-driven development (TDD) approach.

---

## 🧩 Architecture Overview

```text
                     +-------------------+
                     |      Client       |
                     |-------------------|
                     | Upload/Download   |
                     | File Operations   |
                     +--------+----------+
                              |
                              |
                     Metadata & Chunk Info
                              |
                              v
                     +-------------------+
                     |      Master       |
                     |-------------------|
                     | Namespace Mgmt    |
                     | Chunk Mapping     |
                     | Heartbeats        |
                     +--+---+---+---+----+
                        |   |   |   |    |
            +-----------+   |   |   |    +------------+
            |               |   |   |                 |
            v               v   v   v                 v
    +----------------+ +----------------+        +----------------+
    | Chunk Server 1 | | Chunk Server 2 |  ...   | Chunk Server N |
    |----------------| |----------------|        |----------------|
    | Store Blocks   | | Store Blocks   |        | Store Blocks   |
    | Send Heartbeat | | Send Heartbeat |        | Send Heartbeat |
    +----------------+ +----------------+        +----------------+

```
---

### 🗂️ Project Directory Structure

```text
gfs-rust/
├── Cargo.toml         # Workspace manifest for all crates
├── Cargo.lock         # Dependency lockfile
├── crates/
│   ├── master/        # Master server: handles metadata & chunk mapping
│   ├── chunk_server/  # Chunk server: stores actual file blocks
│   ├── client/        # Client: uploads/downloads files, interacts with master
│   ├── shared/        # Shared data types, messages, serialization logic
│   └── tests/         # System and integration tests (multi-node simulation)
```
