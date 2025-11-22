# Agent Development Guide for Gait

## Build/Test Commands

- **Build**: `cargo build`
- **Release build**: `cargo build --release`
- **Check**: `cargo check`
- **Format**: `cargo fmt`
- **Lint**: `cargo clippy --all-targets --all-features -- -D warnings`
- **Test all**: `cargo test`
- **Test single**: `cargo test test_name` or `cargo test --package gait --lib test_name`
- **Test specific module**: `cargo test --lib tui::app::tests::test_regeneration_adds_new_message`

## Code Style Guidelines

### Imports & Organization

- Group imports: std, external crates, then local modules
- Use absolute paths for clarity: `crate::core::context::CommitContext`
- Avoid wildcard imports (`use crate::*`)

### Naming Conventions

- **Functions**: snake_case (`get_user_info()`)
- **Types/Structs**: PascalCase (`CommitContext`)
- **Constants**: SCREAMING_SNAKE_CASE (`MAX_RETRY_COUNT`)
- **Fields**: snake_case (`user_name`, `is_active`)

### Error Handling

- Use `anyhow::Result<T>` for fallible operations
- Prefer `?` operator for error propagation
- Add context with `.context("descriptive message")`
- Avoid `unwrap()`/`expect()` in production code

### Types & Data Structures

- Use `&str` for immutable string parameters
- Prefer `String` only when ownership needed
- Use `Vec<T>` for dynamic arrays, `VecDeque<T>` for queues
- Always derive `Debug` on custom types
- Use strong typing with enums for variants

### Formatting & Style

- Use inline format args: `format!("{user} logged in")` not `format!("{} logged in", user)`
- 4-space indentation
- Line length: ~100 characters
- Use `String::with_capacity()` when size is known

### Async Code

- Use `tokio` runtime for async operations
- Prefer `async fn` for async functions
- Use `tokio::spawn` for background tasks
- Handle async errors properly

### Testing

- Write unit tests for all public functions
- Use descriptive test names: `test_regeneration_adds_new_message`
- Test edge cases and error conditions
- Use `#[cfg(test)]` for test-only code

### Performance

- Avoid allocations in hot paths
- Use zero-copy operations with slices
- Pre-allocate collections when possible
- Profile with `cargo flamegraph` if needed

## Architecture Patterns

- Async I/O with Tokio for non-blocking operations
- Input validation on all external data
- Strong typing with Rust's type system
- Error context propagation with `anyhow`
