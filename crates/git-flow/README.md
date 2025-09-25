# 🔮 Git-Iris: AI-Powered Git Workflow Assistant

<div align="center">

[![CI/CD](https://img.shields.io/github/actions/workflow/status/hyperb1iss/git-iris/cicd.yml?style=for-the-badge&logo=github-actions&logoColor=white&color=4C566A)](https://github.com/hyperb1iss/git-iris/actions)
[![Docker](https://img.shields.io/docker/pulls/hyperb1iss/git-iris?style=for-the-badge&logo=docker&logoColor=white&color=2496ED)](https://hub.docker.com/r/hyperb1iss/git-iris)
[![License](https://img.shields.io/badge/License-Apache%202.0-5E81AC?style=for-the-badge&logo=apache&logoColor=white&color=3B6EA8)](https://opensource.org/licenses/Apache-2.0)
[![GitHub Release](https://img.shields.io/github/release/hyperb1iss/git-iris.svg?style=for-the-badge&logo=github&logoColor=white&color=9D6DB3)][releases]
[![Crates.io](https://img.shields.io/crates/v/git-iris.svg?style=for-the-badge&logo=rust&logoColor=white&color=D35D47)][crates]
[![Rust](https://img.shields.io/badge/rust-stable-EBCB8B?style=for-the-badge&logo=rust&logoColor=white&color=EFBB4D)](https://www.rust-lang.org/)
[![ko-fi](https://img.shields.io/badge/Ko--fi-Support%20Me-A3BE8C?style=for-the-badge&logo=ko-fi&logoColor=white&color=82B062)](https://ko-fi.com/hyperb1iss)

_Elevate your Git workflow with the power of AI_ 🚀

[Installation](#installation) • [Docker](#docker) • [Configuration](#configuration) • [Usage](#usage) • [Contributing](#contributing) • [License](#license)

</div>

<div align="center">
  <img src="https://raw.githubusercontent.com/hyperb1iss/git-iris/main/docs/images/git-iris-screenshot-1.png" alt="Git-Iris Screenshot 1" width="33%">
  <img src="https://raw.githubusercontent.com/hyperb1iss/git-iris/main/docs/images/git-iris-screenshot-2.png" alt="Git-Iris Screenshot 2" width="33%">
  <img src="https://raw.githubusercontent.com/hyperb1iss/git-iris/main/docs/images/git-iris-screenshot-3.png" alt="Git-Iris Screenshot 3" width="33%">
</div>

_Git-Iris in action: AI-powered Git workflow assistance_

## 🌟 Overview

Git-Iris is a comprehensive AI-powered Git workflow assistant that enhances your development process from start to finish. It offers intelligent support for crafting meaningful commit messages, generating insightful changelogs, creating detailed release notes, and providing code reviews. By leveraging advanced AI models, Git-Iris boosts your productivity and improves the quality of your project documentation.

## ✨ Features

Git-Iris offers a suite of AI-powered tools to enhance your Git workflow:

### 🧠 Core Features

- **Intelligent Commit Messages**: Generate context-aware, meaningful commit messages
- **AI-Powered Code Reviews**: Get detailed feedback on your changes across multiple quality dimensions
- **Dynamic Changelog Generation**: Create structured changelogs between any Git references
- **Comprehensive Release Notes**: Generate release notes with summaries and key changes
- **MCP Integration**: Connect directly with Claude, Cursor, VSCode and other compatible AI tools
- **Project-Specific Configuration**: Maintain shared project settings in version control

### 💪 Advanced Capabilities

- **Multi-Provider Support**: Works with OpenAI, Anthropic, Google, Groq, XAI, Ollama and more
- **Remote Repository Support**: Work with remote repositories without manual cloning
- **Customizable Workflows**: Control AI behavior with custom instructions and presets
- **Smart Context Analysis**: Extract relevant context from repository changes
- **Multi-Language Support**: Analyze code in Rust, JavaScript, Python, Java, and more languages

### 🎨 User Experience

- **Interactive CLI**: Refine AI-generated content through an intuitive interface
- **Gitmoji Integration**: Add expressive emojis to your Git documentation
- **Performance Optimized**: Efficient token management for responsive interactions
- **Docker Support**: Run in CI/CD pipelines and workflows without installation

## 🛠️ Installation

### Prerequisites

- Rust and Cargo (latest stable version)
- Git 2.23.0 or newer

### Via Cargo (Recommended)

```bash
cargo install git-iris
```

### Via Docker

Git-Iris is available as a Docker image:

```bash
docker pull hyperb1iss/git-iris:latest
```

Run it:

```bash
docker run --rm -v "$(pwd):/git-repo" hyperb1iss/git-iris gen
```

For detailed instructions, examples, and CI/CD integration, see our [Docker Usage Guide](docker/README.md).

### Manual Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/hyperb1iss/git-iris.git
   cd git-iris
   ```

2. Build and install:
   ```bash
   cargo build --release
   cargo install --path .
   ```

## 🐳 Docker

Git-Iris provides official Docker images for easy integration into your CI/CD pipelines and workflows without installation:

```bash
# Generate a commit message (mount current directory)
docker run --rm --user $(id -u):$(id -g) -v "$(pwd):/git-repo" hyperb1iss/git-iris gen

# Configure with environment variables
docker run --rm --user $(id -u):$(id -g) -v "$(pwd):/git-repo" \
  -e GITIRIS_PROVIDER="openai" \
  -e GITIRIS_API_KEY="your-api-key" \
  hyperb1iss/git-iris gen
```

For persistent configuration, mount a volume to store your settings:

```bash
docker run --rm --user $(id -u):$(id -g) -v "$(pwd):/git-repo" \
  -v git-iris-config:/root/.config/git-iris \
  hyperb1iss/git-iris config --provider openai --api-key your-api-key
```

The Docker image is particularly useful in CI/CD workflows:

```yaml
# GitHub Actions example
- name: Generate Release Notes
  env:
    GITIRIS_PROVIDER: openai
    GITIRIS_API_KEY: ${{ secrets.OPENAI_API_KEY }}
  run: |
    docker run --rm -v "$(pwd):/git-repo" \
      -e GITIRIS_PROVIDER -e GITIRIS_API_KEY \
      hyperb1iss/git-iris release-notes \
      --from $(git describe --tags --abbrev=0 $(git rev-list --tags --skip=1 --max-count=1)) \
      --to $(git describe --tags --abbrev=0) \
      --print > RELEASE_NOTES.md
```

To build and test the Docker image locally:

```bash
# Build the image with a custom tag
./docker/build.sh mytag

# Test the image
./docker/test-image.sh mytag
```

For detailed instructions, examples, and CI/CD integration, see our [Docker Usage Guide](docker/README.md).

## ⚙️ Configuration

Git-Iris offers both global configuration and project-specific configuration options.

### Global Configuration

Global settings are stored in `~/.config/git-iris/config.toml` and apply across all repositories:

```bash
# For OpenAI
git-iris config --provider openai --api-key YOUR_OPENAI_API_KEY

# For Anthropic
git-iris config --provider anthropic --api-key YOUR_ANTHROPIC_API_KEY
# Note: "claude" is still supported for backward compatibility

# For Google
git-iris config --provider google --api-key YOUR_GOOGLE_API_KEY

# For Ollama (no API key required)
git-iris config --provider ollama

# For other supported providers (Groq, XAI, DeepSeek, Phind)
git-iris config --provider <provider> --api-key YOUR_API_KEY
```

### Project-Specific Configuration

Project settings are stored in `.irisconfig` in your repository root and can be shared with your team without sharing sensitive credentials:

```bash
# Set project-specific LLM provider
git-iris project-config --provider anthropic

# Configure project-specific preset
git-iris project-config --preset conventional

# Set model for the project
git-iris project-config --model claude-3-7-sonnet-20250219

# View current project configuration
git-iris project-config --print
```

Project configuration files do not store API keys for security reasons but can store other settings like models, presets, and custom instructions.

### Supported LLM Providers

Git-Iris supports the following LLM providers:

| Provider  | Default Model              | Token Limit | API Key Required |
| --------- | -------------------------- | ----------- | ---------------- |
| anthropic | claude-3-7-sonnet-20250219 | 200,000     | Yes              |
| deepseek  | deepseek-chat              | 64,000      | Yes              |
| google    | gemini-2.0-flash           | 1,000,000   | Yes              |
| groq      | llama-3.1-70b-versatile    | 128,000     | Yes              |
| ollama    | llama3                     | 128,000     | No               |
| openai    | gpt-4o                     | 128,000     | Yes              |
| phind     | phind-v2                   | 32,000      | No               |
| xai       | grok-2-beta                | 128,000     | Yes              |

Additional configuration options:

```bash
# Enable/Disable Gitmoji
git-iris config --gitmoji true

# Set custom instructions
git-iris config --instructions "Always mention the ticket number in the commit message"

# Set default instruction preset
git-iris config --preset conventional

# Set token limit for a provider
git-iris config --provider openai --token-limit 4000

# Set model for a provider
git-iris config --provider openai --model gpt-4o

# Set additional parameters for a provider
git-iris config --provider openai --param temperature=0.7 --param max_tokens=150
```

For more detailed configuration information, please refer to our [Configuration Guide](CONFIG.md).

## 📖 Usage

### Global Options

These options apply to all commands:

- `-l`, `--log`: Log debug messages to a file
- `--log-file`: Specify a custom log file path
- `-q`, `--quiet`: Suppress non-essential output (spinners, waiting messages, etc.)
- `-v`, `--version`: Display version information
- `-r`, `--repo`: Use a remote repository URL instead of local repository

### Generate Commit Messages

Generate an AI-powered commit message:

```bash
git-iris gen
```

Options:

- `-a`, `--auto-commit`: Automatically commit with the generated message
- `-i`, `--instructions`: Provide custom instructions for this commit
- `--provider`: Specify an LLM provider
- `--preset`: Use a specific instruction preset
- `--no-gitmoji`: Disable Gitmoji for this commit
- `-p`, `--print`: Print the generated message to stdout and exit
- `--no-verify`: Skip verification steps (pre/post commit hooks)

Example:

```bash
git-iris cmsg -a -i "Focus on performance improvements" --provider anthropic --preset detailed
```

To generate a commit message and print it to stdout without starting the interactive process:

```bash
git-iris cmsg --print
```

### AI-Powered Code Reviews

Get a comprehensive analysis of your staged changes:

```bash
git-iris review
```

Options:

- `-i`, `--instructions`: Provide custom instructions for this review
- `--provider`: Specify an LLM provider
- `--preset`: Use a specific instruction preset
- `-p`, `--print`: Print the generated review to stdout and exit
- `--include-unstaged`: Include unstaged changes in the review
- `--commit`: Review a specific commit by ID (hash, branch, or reference)

Example:

```bash
git-iris review -i "Focus on security" --preset security --include-unstaged
```

For more comprehensive reviews, you can also:

- **Review unstaged changes**: Include unstaged changes in the review
  ```bash
  git-iris review --include-unstaged
  ```

- **Review a specific commit**: Analyze a particular commit
  ```bash
  git-iris review --commit abc123
  ```

- **Review branch differences**: Compare entire branches (perfect for PR reviews)
  ```bash
  git-iris review --from main --to feature-branch
  ```

The branch comparison feature is particularly powerful for reviewing pull requests, as it analyzes all changes between two branches, giving you a comprehensive view of the entire feature or fix.

**Smart Branch Comparison**: Git-Iris uses merge-base comparison to ensure you only see changes relevant to the feature branch, not unrelated changes that happened in the base branch after the feature branch was created. This provides accurate PR reviews even when the base branch has moved forward.

Example usage:

```bash
# Compare feature branch to main (explicit)
git-iris review --from main --to feature-branch

# Compare feature branch to main (using default)
git-iris review --to feature-branch  # --from defaults to 'main'
```

The review includes:

- A summary of the changes
- Code quality assessment
- Suggestions for improvement
- Identified issues
- Positive aspects of your code
- Analysis across 11 dimensions of code quality:
  - Complexity - Identifies unnecessary complexity in algorithms and control flow
  - Abstraction - Assesses appropriateness of abstractions and design patterns
  - Unintended Deletion - Detects critical functionality removed without replacement
  - Hallucinated Components - Flags references to non-existent functions or APIs
  - Style Inconsistencies - Highlights deviations from project coding standards
  - Security Vulnerabilities - Identifies potential security issues
  - Performance Issues - Spots inefficient algorithms or resource usage
  - Code Duplication - Detects repeated logic or copy-pasted code
  - Error Handling - Evaluates completeness of error recovery strategies
  - Test Coverage - Analyzes test coverage gaps or brittle tests
  - Best Practices - Checks adherence to language-specific conventions and design guidelines

### Generate Pull Request Descriptions

Create comprehensive PR descriptions for changesets spanning multiple commits or single commits:

```bash
git-iris pr --from <from-ref> --to <to-ref>
```

Options:

- `--from`: Starting Git reference (commit hash, tag, or branch name)
- `--to`: Ending Git reference (defaults to HEAD if not specified)
- `-i`, `--instructions`: Custom instructions for PR description generation
- `--preset`: Select an instruction preset for PR description generation
- `-p`, `--print`: Print the generated PR description to stdout and exit

**Examples:**

**Single commit analysis:**
```bash
# Analyze a single commit (compares against its parent)
git-iris pr --from abc1234
git-iris pr --to abc1234

# Analyze a specific commitish (e.g., 2 commits ago)
git-iris pr --to HEAD~2

# Same commit for both from and to
git-iris pr --from abc1234 --to abc1234
```

**Multiple commit analysis:**
```bash
# Review the last 3 commits together
git-iris pr --from HEAD~3

# Review commits from a specific point to now
git-iris pr --from @~5
```

**Commit range analysis:**
```bash
git-iris pr --from v1.0.0 --to HEAD --preset detailed
```

**Branch comparison:**
```bash
git-iris pr --from main --to feature-auth --preset conventional
```

**Comparison to main:**
```bash
# Compare feature branch to main
git-iris pr --to feature-branch
```

The PR description generator analyzes the entire changeset as an atomic unit rather than individual commits, providing:

- A comprehensive title and summary
- Detailed description of what was changed and why
- List of commits included in the PR
- Breaking changes identification
- Testing notes and deployment considerations
- Technical implementation details

This is particularly useful for:
- **Single commits**: Get detailed analysis of what changed in a specific commit or commitish (e.g., `HEAD~2`)
- **Multiple commits**: Review a range of commits together (e.g., `--from HEAD~3` reviews the last 3 commits)
- **Feature branches**: Get a complete overview of all changes in a feature
- **Release preparations**: Understand what's included in a release candidate
- **Code reviews**: Provide reviewers with comprehensive context
- **Documentation**: Create detailed records of what changed between versions

**Supported commitish syntax**: `HEAD~2`, `HEAD^`, `@~3`, `main~1`, `origin/main^`, and other Git commitish references.

### Generate Changelogs

Create a detailed changelog between Git references:

```bash
git-iris changelog --from <from-ref> --to <to-ref>
```

Options:

- `--from`: Starting Git reference (commit hash, tag, or branch name)
- `--to`: Ending Git reference (defaults to HEAD if not specified)
- `-i`, `--instructions`: Custom instructions for changelog generation
- `--preset`: Select an instruction preset for changelog generation
- `--detail-level`: Set the detail level (minimal, standard, detailed)
- `--gitmoji`: Enable or disable Gitmoji in the changelog
- `--update`: Update the changelog file with the new changes
- `--file`: Path to the changelog file (defaults to CHANGELOG.md)
- `--version-name`: Explicit version name to use in the changelog instead of getting it from Git

Example:

```bash
git-iris changelog --from v1.0.0 --to v1.1.0 --detail-level detailed --update
```

Example using explicit version name (useful in release workflows):

```bash
git-iris changelog --from v1.0.0 --to HEAD --update --version-name v1.1.0
```

### Generate Release Notes

Create comprehensive release notes:

```bash
git-iris release-notes --from <from-ref> --to <to-ref>
```

Options:

- `--from`: Starting Git reference (commit hash, tag, or branch name)
- `--to`: Ending Git reference (defaults to HEAD if not specified)
- `-i`, `--instructions`: Custom instructions for release notes generation
- `--preset`: Select an instruction preset for release notes generation
- `--detail-level`: Set the detail level (minimal, standard, detailed)
- `--gitmoji`: Enable or disable Gitmoji in the release notes
- `--version-name`: Explicit version name to use in the release notes instead of getting it from Git

Example:

```bash
git-iris release-notes --from v1.0.0 --to v1.1.0 --preset conventional
```

Example using explicit version name:

```bash
git-iris release-notes --from v1.0.0 --to HEAD --version-name v1.1.0
```

### Project Configuration

Create or update project-specific settings:

```bash
git-iris project-config
```

Options:

- `--provider`: Set default LLM provider for this project
- `--model`: Set model for the specified provider
- `--token-limit`: Set token limit for the specified provider
- `--param`: Set additional parameters for the specified provider
- `-p`, `--print`: Print the current project configuration
- `--gitmoji`: Enable or disable Gitmoji for this project
- `-i`, `--instructions`: Set instructions for message generation
- `--preset`: Set default instruction preset for this project

Example:

```bash
git-iris project-config --provider anthropic --preset security --model claude-3-7-sonnet-20250219
```

### Working with Remote Repositories

Git-Iris supports working with remote repositories directly without having to clone them manually:

```bash
# Generate a changelog between two tags on a remote repository
git-iris changelog --repo https://github.com/example/repo.git --from v1.0.0 --to v2.0.0

# Generate release notes for a remote repository
git-iris release-notes --repo https://github.com/example/repo.git --from v1.0.0 --to v2.0.0

# Review code in a remote repository (read-only)
git-iris review --repo https://github.com/example/repo.git

# Generate a commit message for a remote repository (read-only)
git-iris cmsg --repo https://github.com/example/repo.git --print
```

Note: When working with remote repositories, Git-Iris operates in read-only mode. You can't commit changes directly to remote repositories.

### MCP Server for AI Integration

Start an MCP (Model Context Protocol) server for integration with AI tools:

```bash
git-iris serve
```

Options:

- `--dev`: Enable development mode with more verbose logging
- `-t`, `--transport`: Transport type to use (stdio, sse)
- `-p`, `--port`: Port to use for network transports
- `--listen-address`: Listen address for network transports

Example:

```bash
git-iris serve --transport sse --port 3077 --listen-address 127.0.0.1
```

This allows you to use Git-Iris features directly from Claude, Cursor, VSCode, and other MCP-compatible tools. See [MCP.md](docs/MCP.md) for detailed documentation.

#### MCP Tool Parameters

All MCP tools **require** the `repository` parameter, which must be a local project path or a remote repository URL. This is necessary because some clients (like Cursor) do not reliably provide the project root.

**Example (local path):**

```json
{
  "from": "v1.0.0",
  "to": "v2.0.0",
  "detail_level": "detailed",
  "repository": "/home/bliss/dev/myproject"
}
```

**Example (remote URL):**

```json
{
  "from": "v1.0.0",
  "to": "v2.0.0",
  "detail_level": "detailed",
  "repository": "https://github.com/example/repo.git"
}
```

**Why is `repository` required?**

> Due to limitations in some MCP clients, the server cannot reliably infer the project root or repository path. To ensure Git-Iris always operates on the correct repository, you must explicitly specify the `repository` parameter for every tool call. This eliminates ambiguity and ensures your commands are always precise and predictable.

### Interactive Commit Process

The interactive CLI allows you to refine and perfect your commit messages:

- Use arrow keys to navigate through suggestions
- Press 'e' to edit the current message
- Press 'i' to modify AI instructions
- Press 'g' to change the emoji
- Press 'p' to change the preset
- Press 'u' to edit user info
- Press 'r' to regenerate the message
- Press Enter to commit
- Press Esc to cancel

## 🎛️ Custom Instructions and Presets

Git-Iris offers two powerful ways to guide the AI in generating commit messages: custom instructions and presets.

### Instruction Presets

Presets are predefined sets of instructions that provide a quick way to adjust the commit message style. Git-Iris comes with several built-in presets to suit different commit styles and project needs.

To list available presets:

```bash
git-iris list-presets
```

This will display a list of all available presets with a brief description of each, categorized by type (general, review-specific, or commit-specific).

Some key presets include:

- `default`: Standard professional style (for both commits and reviews)
- `conventional`: Follows the Conventional Commits specification
- `detailed`: Provides more context and explanation
- `concise`: Short and to-the-point responses
- `cosmic`: Mystical, space-themed language

For code reviews specifically, Git-Iris includes specialized presets:

- `security`: Focus on security vulnerabilities and best practices
- `performance`: Analyze code for performance optimizations
- `architecture`: Evaluate architectural patterns and design decisions
- `testing`: Focus on test coverage and testing strategies
- `maintainability`: Evaluate code for long-term maintenance
- `conventions`: Check adherence to language and project coding standards

To use a preset for a single commit:

```bash
git-iris cmsg --preset conventional
```

To use a preset for a code review:

```bash
git-iris review --preset security
```

To set a default preset:

```bash
git-iris config --preset conventional
```

Presets work seamlessly with other Git-Iris features. For example, if you have Gitmoji enabled, the preset instructions will be applied in addition to adding the appropriate Gitmoji.

### Custom Instructions

Custom instructions allow you to provide specific guidance for commit message generation. These can be set globally or per-commit.

Setting global custom instructions:

```bash
git-iris config --instructions "Always include the ticket number and mention performance impacts"
```

Providing per-commit instructions:

```bash
git-iris cmsg -i "Emphasize security implications of this change"
```

### Combining Presets and Custom Instructions

When using both a preset and custom instructions, Git-Iris combines them, with custom instructions taking precedence. This allows you to use a preset as a base and fine-tune it with specific instructions.

```bash
git-iris cmsg --preset conventional -i "Mention the JIRA ticket number"
```

In this case, the commit message will follow the Conventional Commits format and include the JIRA ticket number.

If you've set a default preset in your configuration, you can still override it for individual commits:

```bash
git-iris cmsg --preset detailed -i "Focus on performance improvements"
```

This will use the 'detailed' preset instead of your default, along with the custom instruction.

### Examples of Custom Instructions

1. **Ticket Number Integration**

   ```
   Always start the commit message with the JIRA ticket number in square brackets
   ```

2. **Language-Specific Conventions**

   ```
   For Rust files, mention any changes to public APIs or use of unsafe code
   ```

3. **Team-Specific Guidelines**

   ```
   Follow the Angular commit message format: <type>(<scope>): <subject>
   ```

4. **Project-Specific Context**

   ```
   For the authentication module, always mention if there are changes to the user model or permissions
   ```

5. **Performance Considerations**
   ```
   Highlight any changes that might affect application performance, including database queries
   ```

Custom instructions and presets allow you to tailor Git-Iris to your specific project needs, team conventions, or personal preferences. They provide a powerful way to ensure consistency and capture important context in your commit messages.

## 🤝 Contributing

We welcome contributions! Please see our [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to get started, our code of conduct, and the process for submitting pull requests.

## 📄 License

Distributed under the Apache 2.0 License. See `LICENSE` for more information.

---

<div align="center">

📚 [Documentation](https://github.com/hyperb1iss/git-iris/wiki) • 🐛 [Report Bug](https://github.com/hyperb1iss/git-iris/issues) • 💡 [Request Feature](https://github.com/hyperb1iss/git-iris/issues)

</div>

<div align="center">

Created by [Stefanie Jane 🌠](https://github.com/hyperb1iss)

If you find Git-Iris useful, [buy me a Monster Ultra Violet](https://ko-fi.com/hyperb1iss)! ⚡️

</div>

[crates-shield]: https://img.shields.io/crates/v/git-iris.svg?style=for-the-badge&logo=rust&logoColor=white&color=D35D47
[crates]: https://crates.io/crates/git-iris
[releases-shield]: https://img.shields.io/github/release/hyperb1iss/git-iris.svg?style=for-the-badge&logo=github&logoColor=white&color=9D6DB3
[releases]: https://github.com/hyperb1iss/git-iris/releases
[license-shield]: https://img.shields.io/github/license/hyperb1iss/git-iris.svg?style=for-the-badge&logo=apache&logoColor=white&color=3B6EA8
