# notizia

Message passing in Rust. Provides a custom task abstraction with mailbox-based communication and macro-based task spawning.

## Overview

This project demonstrates message passing patterns in Rust using:
- `std::sync::mpsc` channels for inter-thread communication
- Custom `Task` and `Mailbox` abstractions
- Macro-based task spawning with `proc!` and `recv!`

## Example

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

## Building

```bash
cargo build
```

## Running Examples

```bash
cargo run --example simple
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
