# GWTFlow

AI-powered Git workflow assistant for smart commits, code reviews, changelogs, and release notes.

## Structure

This project produces multiple executables from a single codebase:

- `git-flow` - Main entry point with all commands available as subcommands
- `git-flow-msg` - Generate a commit message using AI
- `git-flow-review` - Review staged changes using AI
- `git-flow-pr` - Generate a pull request description using AI
- `git-flow-changelog` - Generate a changelog
- `git-flow-release-notes` - Generate release notes
- `git-flow-serve` - Start an MCP server
- `git-flow-config` - Configure Git-Iris settings and providers
- `git-flow-project` - Manage project-specific configuration
- `git-flow-list-presets` - List available instruction presets

## Building

To build all executables:

```bash
cargo build --release
```

The executables will be located in `target/release/`.

## Usage

With the main executable:
```bash
git-flow msg [OPTIONS]
git-flow review [OPTIONS]
git-flow pr [OPTIONS]
# etc.
```

With individual executables:
```bash
git-flow-msg [OPTIONS]
git-flow-review [OPTIONS]
git-flow-pr [OPTIONS]
# etc.
```