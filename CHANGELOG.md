# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] -

### Added

- Add `CHANGELOG.md` to document project evolution and release notes (f64fa5)

### Changed

- Consolidate file analysis logic into the `analyzer` module, moving implementations from `src/file_analyzers/` and updating import paths (1a259d)
- Update project version in `Cargo.toml` from 0.1.0 to 0.1.1 (f64fa5)
- Adjust `README.md` header from 'Wire operations (caching, syncing)' to 'Wire operations (syncing)' (f64fa5)

### Removed

- Remove the `git-serve` command and all related server modules and files, including `src/bin/serve.rs` and the entire `src/server` directory (d87fc5)
- Remove unused `use_emoji` and `instruction_preset` configurations from `Config` and delete `src/instruction_presets.rs` (077f09)
- Remove `EditingUserInfo` mode and related handling from the TUI, simplifying its state and input handling (2bfc62)
- Remove unused `use` statements in `src/config.rs` to improve code readability (0e16c3)

### ⚠ Breaking Changes

- The `git-serve` command and its associated server functionality have been removed, making it unavailable for use (d87fc5)

### 📊 Metrics

- Total Commits: 6
- Files Changed: 73
- Insertions: 2129
- Deletions: 4879
