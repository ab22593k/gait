# Ritex

Ritex is a Rust workspace project that provides AI-powered Git workflow assistance and specification-driven development tools.

## Project Structure

- `/crates/` - Contains the main Rust crates:
  - `git-iris` - AI-powered Git workflow assistant
  - `git-wire` - Git subcommand for declarative cross-repository code synchronization
- `/src/` - Main entry point
- `/commands/` - AI command workflow definitions (TOML files)
- `/.specify/` - Specification and planning system templates and scripts

## Crates

### git-wire

A tool that wires parts of other repositories' source code into the current repository in a declarative manner. Features include:

- Declarative cross-repository code synchronization
- JSON-based configuration for managing external code dependencies
- Multiple checkout methods (shallow, shallow_no_sparse, partial)
- Multi-threaded execution with single-threaded option
- Repository caching to avoid multiple git pulls for the same repository

### git-iris

An AI-powered Git workflow assistant that enhances development processes with intelligent support for commit messages, code reviews, changelogs, and release notes.

## Building

To build all crates:

```bash
cargo build
```

To build in release mode:

```bash
cargo build --release
```

To build a specific crate:

```bash
cargo build -p git-wire
```

## License

Licensed under various open-source licenses (Apache-2.0 for git-iris, MIT for git-wire).