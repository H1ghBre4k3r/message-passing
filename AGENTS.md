# AGENTS.md

This document provides essential information for agents working with the notizia codebase - a Rust message passing library with both synchronous and asynchronous implementations.

## Project Overview

notizia is a Rust library providing message passing abstractions using:
- `std::sync::mpsc` for synchronous inter-thread communication
- `tokio::sync::mpsc` for asynchronous message passing (opt-in via feature flag)
- Custom `Task`/`AsyncTask` and `Mailbox`/`AsyncMailbox` abstractions
- Macro-based task spawning with `proc!`/`async_proc!` and `recv!`

## Essential Commands

### Building
```bash
# Standard build
cargo build

# Build with tokio feature
cargo build --features tokio

# Check without building
cargo check
cargo check --features tokio
```

### Testing
```bash
# Run all tests (std implementation only)
cargo test --verbose

# Run tests with tokio feature
cargo test --features tokio --verbose

# Run specific test
cargo test test_basic_task_communication

# Run async tests
cargo test --features tokio --test tokio_impl
```

### Code Quality
```bash
# Format code
cargo fmt

# Check formatting (CI uses this)
cargo fmt --all -- --check

# Run linter (strict: warnings as errors)
cargo clippy -- -D warnings
cargo clippy --features tokio -- -D warnings
```

### Examples
```bash
# Run synchronous example
cargo run --example simple

# Run asynchronous example (requires tokio feature)
cargo run --example async --features tokio
```

### Documentation
```bash
# Generate and open documentation
cargo doc --open
```

## Code Organization

```
notizia/
├── src/
│   ├── lib.rs           # Entry point, re-exports std_impl and tokio_impl
│   ├── std_impl.rs      # Synchronous implementation (std::sync::mpsc)
│   └── tokio_impl.rs    # Async implementation (tokio::sync::mpsc) [tokio feature]
├── examples/
│   ├── simple.rs        # Synchronous example
│   └── async.rs         # Async example
└── Cargo.toml           # Project manifest with features
```

### Module Structure

**lib.rs** - Main entry point:
- Declares `std_impl` module and re-exports all its contents
- Conditionally includes `tokio_impl` module when `tokio` feature is enabled
- Re-exports `tokio` crate for async API consumers

**std_impl.rs** - Synchronous API:
- `Mailbox<T>`: Internal struct wrapping `std::sync::mpsc::Sender<T>` (private)
- `Task<M, R>`: Public task type with message type `M` and return type `R`
- `spawn_task<M, R, Func>(func) -> Task<M, R>`: Spawns a new task
- `proc!` macro: User-friendly task creation syntax
- `recv!` macro: Message receiving inside tasks
- All tests in `#[cfg(test)] mod tests`

**tokio_impl.rs** - Asynchronous API:
- `AsyncMailbox<T>`: Internal struct wrapping `tokio::sync::mpsc::Sender<T>` (private)
- `AsyncTask<M, R>`: Public async task type
- `spawn_async_task<M, R, Output, Func>(func) -> AsyncTask<M, Output>`: Spawns async task
- `async_proc!` macro: User-friendly async task creation syntax
- `recv!` macro: Async message receiving (overloaded macro name)
- All tests in `#[cfg(test)] mod tests` with `#[tokio::test]`

## Code Patterns and Conventions

### Parallel API Design

The library maintains two parallel implementations with nearly identical APIs:

**Synchronous (std)**:
```rust
Task<M, R>::send(&self, payload: T)          // Blocking send
Task<M, R>::join(self) -> R                 // Blocking join
```

**Asynchronous (tokio)**:
```rust
AsyncTask<M, R>::send(&self, payload: T)    // Returns Future
AsyncTask<M, R>::join(self) -> R            // Returns Future (self consumed)
```

### Trait Bounds

Both implementations require:
- Message type `M`: `Send + 'static`
- Return type `R`: `Send + 'static` (async requires `Send + 'static + Future<Output = Output>`)
- Function closure: `Send + 'static`

### Channel Buffer Sizes

- **std**: Uses unbounded channel (`channel::<M>()`)
- **tokio**: Uses bounded channel with capacity 64 (`channel::<M>(64)`)

### Macro Patterns

**Task Creation Macros**:
- `proc! { ... }`: Creates synchronous task, defines `recv!` macro internally
- `async_proc! { ... }`: Creates async task, defines `recv!` macro internally

Both macros:
- Take arbitrary token tree `($($content:tt)*)`
- Define a local `recv!` macro that receives messages
- Call the appropriate `spawn_task`/`spawn_async_task` function

**Message Receiving**:
- `recv!()` macro used inside task bodies
- Expands to appropriate receive call (`.recv().unwrap()` or `.recv().await.unwrap()`)
- Macro is defined locally within each `proc!`/`async_proc!` invocation

### Testing Patterns

Tests are organized inline within implementation files:

**std_impl.rs tests**:
```rust
#[test]
fn test_<description>() {
    let task = spawn_task(|receiver| {
        // task logic
    });

    // Send messages
    task.send(value);

    // Verify result
    assert_eq!(task.join(), expected);
}
```

**tokio_impl.rs tests**:
```rust
#[tokio::test]
async fn test_<description>() {
    let task = spawn_async_task(|mut receiver| async move {
        // async task logic
    });

    // Send messages (async)
    task.send(value).await;

    // Verify result
    assert_eq!(task.join().await, expected);
}
```

Test naming: `test_<behavior_description>` (snake_case)

### Feature Flags

**tokio feature**:
- Disabled by default
- Enables `tokio_impl.rs` module
- Adds tokio dependency
- Exposes async API types and functions
- Both implementations can coexist (tests use both)

Conditional compilation:
```rust
#[cfg(feature = "tokio")]
mod tokio_impl;
```

## Important Gotchas

### Macro Reuse

The `recv!` macro name is used in both `std_impl` and `tokio_impl` but with different implementations:
- Standard: `receiver.recv().unwrap()`
- Tokio: `receiver.recv().await.unwrap()`

This works because the macro is defined locally within each `proc!`/`async_proc!` invocation, not globally.

### Error Handling

Throughout the codebase, errors are handled with `unwrap()`:
- `sender.send(payload).unwrap()` in `send()` methods
- `receiver.recv().unwrap()` via `recv!` macro
- `handle.join().unwrap()` in `join()` methods

This is a deliberate simplification - panic on channel errors or task panics.

### Type Parameters

- `Task<M, R>` and `AsyncTask<M, R>`: `M` is message type, `R` is return type
- `send()` requires `T: Clone` (impl `impl<T, R> Task<T, R>` where `T: Clone`)
- Generic bounds on function parameters ensure proper thread safety

### Async Task Return Types

For `AsyncTask<M, R>`, the generic `R` is a `Future<Output = Output>`:
```rust
R: Send + 'static + Future<Output = Output>,
Output: Send + 'static,
```

This allows returning futures directly or values that are implicitly converted.

### Channel Receiver Patterns

- **std**: Receiver is passed directly (`receiver: Receiver<M>`)
- **tokio**: Receiver is mutable (`mut receiver: Receiver<M>`) because async methods require mutable reference

## CI Pipeline

The `.github/workflows/ci.yml` runs:

1. **Check job**: `cargo check` and `cargo check --features tokio`
2. **Test job**: `cargo test`
3. **Test with tokio job**: `cargo test --features tokio`
4. **Format job**: `cargo fmt --all -- --check`
5. **Clippy job**: `cargo clippy -- -D warnings` and `cargo clippy --features tokio -- -D warnings`

All jobs use `RUST_BACKTRACE=1` environment variable for debugging.

## Code Style Guidelines

- Use `cargo fmt` for formatting (Rust standard)
- Run `cargo clippy -- -D warnings` before committing
- Follow existing naming: `Task`/`AsyncTask`, `proc!`/`async_proc!`, `spawn_task`/`spawn_async_task`
- Keep parallel implementations synchronized when making API changes
- Include tests for new functionality in the appropriate implementation module
- Update examples if changing public API

## Development Workflow

When adding features:

1. Implement in `std_impl.rs` first (synchronous version)
2. Port to `tokio_impl.rs` maintaining API consistency
3. Add tests to both implementations
4. Update `Cargo.toml` if new dependencies are needed
5. Update examples if API changes
6. Run `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --all -- --check`
7. Test both with and without tokio feature

## Project Metadata

- **Edition**: Rust 2024
- **Version**: 0.1.0
- **License**: MIT (with optional Apache-2.0)
- **Repository**: https://github.com/H1ghBre4k3r/notizia
- **Author**: Louis Meyer (H1ghBre4k3r)
