# notizia

Message passing in Rust. Provides a custom task abstraction with mailbox-based communication and macro-based task spawning.

Supports both synchronous (blocking) and asynchronous (tokio-based) task execution.

## Overview

This project demonstrates message passing patterns in Rust using:
- `std::sync::mpsc` channels for synchronous inter-thread communication
- `tokio::sync::mpsc` channels for asynchronous message passing (optional)
- Custom `Task`/`AsyncTask` and `Mailbox`/`AsyncMailbox` abstractions
- Macro-based task spawning with `proc!`/`async_proc!` and `recv!`

## Features

- **Synchronous API**: Use `Task`, `proc!`, and `recv!` for blocking operations
- **Asynchronous API**: Use `AsyncTask`, `async_proc!`, and `recv!` with tokio (requires `tokio` feature)
- Zero-cost abstractions: Async support is opt-in via feature flag

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
notizia = "0.1"
```

For async support:

```toml
[dependencies]
notizia = { version = "0.1", features = ["tokio"] }
tokio = { version = "1", features = ["full"] }
```

## Example

### Synchronous

```rust
use notizia::{Task, proc};

// Create a task that sums up numbers it receives
let task: Task<u32, u32> = proc! {
    let mut total = 0;
    for _ in 0..5 {
        let val = recv!();  // Receive a value
        total += val;
    }
    total  // Return the sum
};

// Send messages to the task
for i in 1..=5 {
    task.send(i);
}

// Join the task and get the result
let result = task.join();
assert_eq!(result, 15);  // 1 + 2 + 3 + 4 + 5 = 15
```

### Asynchronous

```rust
use notizia::{AsyncTask, async_proc};

#[tokio::main]
async fn main() {
    let task: AsyncTask<u32, u32> = async_proc! {
        let mut total = 0;
        for _ in 0..5 {
            let val = recv!();  // Receive a value (async)
            total += val;
        }
        total  // Return the sum
    };

    // Send messages to the task
    for i in 1..=5 {
        task.send(i).await;
    }

    // Join the task and get the result
    let result = task.join().await;
    assert_eq!(result, 15);
}
```

## Building

```bash
cargo build

# With tokio feature
cargo build --features tokio
```

## Running Examples

```bash
cargo run --example simple

# Async example (requires tokio feature)
cargo run --example async --features tokio
```

## Testing

```bash
cargo test
```

## Linting

```bash
cargo clippy
cargo fmt
```

## Documentation

See [AGENTS.md](AGENTS.md) for detailed information about the codebase structure, patterns, and conventions.

Generate API documentation:

```bash
cargo doc --open
```

## License

MIT OR Apache-2.0 
