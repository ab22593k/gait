# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2025-04-17

### ✨ Added

- ✨ Support explicit version name for changelog and release notes with new --version-name option (67d9ead)
- ✨ Add Model Context Protocol (MCP) server integration with stdio and SSE transport options (22cabd4, 72c1651)
- ✨ Implement MCP tools for commit messages, code reviews, changelogs, and release notes (251b070, f64ba1f, 67d0b8d, 891c416)
- ✨ Add remote repository support for working with Git repos without manual checkout (3800d04)
- ✨ Add project configuration command for team-shared settings via .irisconfig files (873c63a)
- ✨ Add changelog file update functionality with --update and --file flags (0bc59d8)
- ✨ Add GenericTextAnalyzer for improved text file support (e2ecaca)
- ✨ Add quiet mode (--quiet/-q) and custom log file options (--log-file) (48db0ca)
- 🐳 Add Docker support with multi-stage build for containerized usage (3db4460)
- 🚀 Add publish workflow to Docker Hub and crates.io in CI/CD pipeline (37007eb)
- 📝 Add project-specific configuration documentation (43d6611)
- 📝 Add detailed changelog entries for versions 0.9.0 through 1.0.1 (46e4ad6)
- ♻️ Refactor git module into specialized submodules for better organization (62e698f)

### 🔄 Changed

- 🔧 Make repository parameter required in MCP tools for improved reliability (c875ece)
- 🔧 Reorganize Cargo.toml structure for better readability and discoverability (b33b61e)
- 📝 Improve package description and keywords in Cargo.toml (d77fd02)
- ♻️ Update RMCP dependency to use released version 0.1.5 instead of git dependency (2273811)

### 🐛 Fixed

- ⬆️ Update dependencies to their latest compatible versions (aff5be1)
- 🔄 Update default LLM models to latest versions (OpenAI gpt-4.1, Anthropic claude-3-7-sonnet-latest) (2a5baf9)

### 📊 Metrics

- Total Commits: 30
- Files Changed: 171
- Insertions: 10405
- Deletions: 2825

<!-- -------------------------------------------------------------- -->

## [1.0.1] - 2025-03-30

### ✨ Added

- ✨ Implement comprehensive 10-dimension code quality analysis system with severity levels, specific locations, detailed explanations, and actionable recommendations (0a29915)
- ✨ Add dedicated waiting messages with cosmic and analytical themes for code reviews (37c921a)
- 🔍 Create QualityDimension enum with new "Best Practices" dimension for centralized quality analysis (e75a648)
- 📝 Add comprehensive documentation for all 11 code quality dimensions (5d7d394)
- 💄 Enhance code review UI with modern styling, decorative Unicode characters, and improved readability (95cd3d5)

### 🔄 Changed

- ⚡️ Optimize regex patterns with Lazy static initialization, eliminating redundant compilations (08debd3)
- ♻️ Refactor code types into dedicated modules for better organization and maintainability (78c6cca)
- ♻️ Refactor JSON parsing with JsonSchema implementation for improved type safety and validation (e77a442)
- 🚀 Release version 1.0.1 (98978d2)

### 🗑️ Removed

- 🔥 Remove String conversion implementations for response types in favor of more robust approaches (e77a442)

### 📊 Metrics

- Total Commits: 9
- Files Changed: 47
- Insertions: 1941
- Deletions: 908

<!-- -------------------------------------------------------------- -->

## [1.0.0] - 2025-03-25

### ✨ Added

- ✨ Add AI-powered code review functionality with structured feedback for staged changes (76bdf31)
- ✨ Add preset type categorization (Commit, Review, Both) for command-specific instruction presets (b8bd6b4)
- 🔄 Migrate to external llm crate for standardized provider handling, supporting additional providers like Groq, XAI, DeepSeek, and Phind (0cbfc40)
- 🎨 Improve commit prompt formatting with statistical summary and better organization of file changes (6fc706a)
- 📝 Add comprehensive documentation for code review and changelog features (3fb5c28)
- 🛡️ Improve error handling with defensive programming patterns throughout the codebase (61cf6c7)
- ⬆️ Update dependencies to latest versions including git2, dirs, colored, rand, and ratatui (46fbe7b)
- ✨ Enhance Git hooks with improved execution environment and proper repository context (88f9f80)
- 💄 Enhance config command with beautifully formatted, colorized output (55bf071)
- 🔧 Modernize CI/CD pipeline with updated GitHub Actions (3388590)
- ⚡️ Set default max_tokens (4096) for LLM requests when not specified (4bb34b6)
- 📝 Update man page with comprehensive documentation for all commands and features (3a67fe9)
- 🎨 Improve CLI interface with better organization and styled provider list (023b8b7)
- 🔄 Add backward compatibility for Claude provider naming (claude → anthropic) (f657841)
- 📝 Add GitHub funding configuration (9098e9f)
- 🔧 Update Rust edition from 2021 to 2024 (c81cd1c)

### 🔄 Changed

- ♻️ Improve config display to preserve instruction formatting with line-by-line output (ff76709)
- 🔍️ Update review prompt to focus on staged changes rather than historical context (ee9de53)
- 🔄 Reorder instruction sections to place user instructions before preset instructions (e74ab66)
- ♻️ Rename LLM interface function from get_refined_message to get_message for simplicity (93abf18)
- 🎨 Reorganize import statements for consistent ordering across the codebase (d3799cb)

### 🐛 Fixed

- 🐛 Fix file content handling for deleted files in review and commit generation (f1d04aa)
- 🔧 Simplify token limit handling across providers for more consistent behavior (c6dbfd1)

### 📊 Metrics

- Total Commits: 27
- Files Changed: 122
- Insertions: 4217
- Deletions: 2083

<!-- -------------------------------------------------------------- -->

## [0.9.0] - 2025-02-24

### ✨ Added

- 🚀 Upgrade to Claude 3.7 Sonnet model with backward compatibility (e4e806c7)
- ✨ Add Python script (scripts/lint.py) to enhance Rust linting and code quality (f6ad5f0e)
- ⚡️ Improve token optimization efficiency with integration in commit service (4e893818)
- ✨ Add Conventional Commits preset to InstructionPresetLibrary (7507a413)
- 📝 Create CHANGELOG.md file to track project history (2cbc567f)
- ♻️ Implement GitRepo struct to encapsulate Git operations (c1f4e5b1)
- 🐛 Add early return for empty input text in apply_gradient function (f66e4ffd)
- 🐛 Improve robustness of parent commit handling in analyze_commit (d895bde1)
- 🚨 Enable Clippy lints for unwrap_used with TODOs for future fixes (ee65a1cc)
- 🚨 Add additional Clippy lints to improve code quality (32f3002f)

### 🔄 Changed

- 🔧 Fine-tune Clippy lint settings for better code clarity and standards (6283d48b)
- 🔧 Update Claude model from 'claude-3-5-sonnet-20240620' to 'claude-3-5-sonnet-20241022' (b4a45bc6)
- ⬆️ Upgrade GitHub Actions artifact handling to v4 (76fca7fa)
- ♻️ Refactor commit message generation process for better readability (e161211a)
- ✅ Replace unwrap() with expect() in test files for better error messages (5be93820)
- 🎨 Apply rustfmt to standardize code style across the project (62a043ed)
- ♻️ Refactor apply_gradient function for better readability (c0e5250a)
- 🔧 Update .gitignore to exclude log files (df9446c9)
- 📝 Update TODO list to reflect current project priorities (44582a00)

### 🐛 Fixed

- 🐛 Fix Clippy lints across multiple files (db008c9b)
- 🚨 Fix Clippy warnings in test files with improved error handling (2196464058)
- ✨ Improve issue and PR extraction with enhanced regex patterns (82b61d3e)

### 🗑️ Removed

- 🔥 Remove unused crates to streamline dependencies (f9fdb81d)

### 📊 Metrics

- Total Commits: 26
- Files Changed: 171
- Insertions: 2565
- Deletions: 1661

<!-- -------------------------------------------------------------- -->

## [0.7.0] -

### 🗑️ Removed

- 🔥 Remove tracking of unstaged files across multiple modules (db9db44)
- 🔥 Delete legacy interactive and old TUI commit modules (630aa21)

### ✨ Added

- ✨ Introduce cosmic-themed TUI for commit message creation (99c9428)
- ✨ Add support for pre and post commit hooks (43c8b56)
- ✨ Implement retry mechanism for LLM requests with exponential backoff (b798758)
- 🚀 Integrate Gitmoji support in TUI for commit messages (217ed78)
- 📝 Create TODO.md file with project roadmap and goals (3e18ffa)
- 🎨 Enhance instruction presets with emojis for visual appeal (7927873)

### 🐛 Fixed

- 🐛 Fix TUI message editing and rendering issues (538552f)
- 🐛 Correct binary file detection in git status parsing (a95c228)
- 🐛 Address CI/CD release issues and improve asset handling (da7b239)

### 🔄 Changed

- ♻️ Refactor project structure for improved modularity and maintainability (f1d60bf, e67206d, b48d37a)
- ⚡️ Optimize performance by parallelizing metadata extraction and caching git context (3a8163d, f1d60bf)
- 🔧 Update logging configuration for flexible log file paths and optional stdout logging (d738d89)
- 📝 Revise README to reflect new Git workflow focus and update project description (c404eb5)

### 📊 Metrics

- Total Commits: 70
- Files Changed: 257
- Insertions: 9691
- Deletions: 6079

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
