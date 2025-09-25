# Git-Iris: TODO List

Git-Iris has made significant progress, with many features already implemented. This list represents current priorities and future enhancements.

## 🛠️ Core Functionality

- [ ] Implement amend support for existing commits
- [ ] Add optional confirmation dialogs before commit and on exit
- [ ] Create a general release manager
- [x] Implement per-project configuration with `.irisconfig`
- [x] Improve preset and custom instructions prompting
- [ ] Implement interactive rebase with Git-Iris
- [ ] Create a commit message template system
- [ ] Add support for signing commits with GPG
- [ ] Improve Git hook handling for better output and failure management
- [ ] Implement pre-commit hook for automated Git-Iris integration

## 🚀 New Features

- [ ] Automatically create commits with changelog summaries for version bumps
- [ ] Develop a VSCode extension for Git-Iris
- [x] MCP integration provides foundation for editor extensions
- [ ] Implement a feature to suggest code improvements based on changes
- [x] Code review system provides foundation for this feature
- [ ] Create a system for managing and applying commit message localization
- [ ] Implement a commit message linter with customizable rules
- [ ] Add support for integrating with issue tracking systems
- [ ] Create a feature for generating code documentation based on changes
- [ ] Pull request generator
  - [ ] Enhance with more repository-specific context
- [ ] Interactive review application for directly applying suggestions
- [ ] Branch analysis command to summarize purpose, changes, and merge status
- [ ] Git plumbing integration for assistance with rebase, cherry-pick, and conflict resolution

## 🔧 Code Quality & Improvements

- [x] Conduct thorough code review
- [ ] Enhance error handling with more granular error types instead of relying solely on `anyhow`
- [ ] Improve regex robustness by replacing `expect()` calls with lazy initialization or proper error handling
- [ ] Refactor TUI async/state handling for more robust state management
- [ ] Centralize configuration logic for layering defaults, file settings, and CLI arguments
- [ ] Add configuration validation during load and save operations
- [ ] Ensure consistent depth of metadata extraction across all language analyzers

## 🧩 Optimizations

- [ ] Implement support for more Git hosting platforms (e.g., GitLab, Bitbucket)
- [ ] Create command-line completion scripts for various shells
- [ ] Implement a diff viewer within the TUI
- [ ] Improve error handling and recovery in LLM API calls
- [ ] Add more comprehensive test coverage for all features
- [ ] Optimize token usage for very large repositories
  - [ ] Explore more sophisticated strategies beyond simple truncation (e.g., summarization)
  - [ ] Make context buffer sizes configurable or dynamically calculated
- [ ] Enhance LLM response parsing with structured output options
- [ ] Provide additional configuration options for MCP server
- [ ] Optimize smarter context selection with file-specific commit history

## 🌐 MCP Integration Enhancements

- [x] Basic MCP server implementation
- [ ] Add additional MCP tools for repository analysis
- [ ] Enhance security and authentication for MCP connections
- [ ] Support more transport protocols for MCP
- [ ] Provide better error handling and fallbacks for MCP tools
- [ ] Create a unified local/remote operation model
- [ ] Develop a web UI or additional IDE extensions

## 📊 Analysis and Visualization

- [ ] Implement a system for analyzing code complexity changes
- [ ] Create a system for managing and applying branch naming conventions
- [ ] Add support for generating commit graphs and visualizations
- [ ] Implement a dashboard for project statistics and insights
- [ ] Create visualizations for code review analysis
- [ ] Integrate linters or static analysis tools into file analyzers
- [ ] Expand metadata extraction (frameworks, libraries, versions) across all file analyzers
- [ ] Enhance the relevance scoring logic for better context prioritization

## 🧪 Research and Experimental Features

- [ ] Explore offline LLM integration for air-gapped environments
- [ ] Investigate fine-tuning options for project-specific models
- [ ] Research machine learning for improved code analysis
- [ ] Experiment with context-aware file analyzers for more languages
- [ ] Develop collaborative features for team-based workflows
- [ ] Systematically review and refine LLM prompts based on results
- [ ] Enhance prompts for the review command to ensure all quality dimensions are covered effectively
- [ ] Utilize advanced LLM features like function calling/tool use and multimodal inputs
- [ ] Design a plugin system for extensibility of analyzers or LLM providers
- [ ] Implement team collaboration features for shared configuration and presets
