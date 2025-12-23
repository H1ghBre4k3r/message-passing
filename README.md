# message-passing

Message passing in Rust. Provides a custom task abstraction with mailbox-based communication and macro-based task spawning.

## Overview

This project demonstrates message passing patterns in Rust using:
- `std::sync::mpsc` channels for inter-thread communication
- Custom `Task` and `Mailbox` abstractions
- Macro-based task spawning with `proc!` and `recv!`

## Building

```bash
cargo build
```

## Running

```bash
cargo run
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

## License

MIT OR Apache-2.0 
