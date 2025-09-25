# 🐳 Using Git-Iris with Docker

This guide explains how to use Git-Iris within Docker containers, making it easy to integrate into your CI/CD pipelines or use across different environments without installation.

## 📦 Docker Images

Git-Iris provides official Docker images available on Docker Hub:

```bash
docker pull hyperb1iss/git-iris:latest
```

Images are tagged with their version and `latest` always points to the most recent release:

- `hyperb1iss/git-iris:latest` - The latest stable release
- `hyperb1iss/git-iris:1.0.1` - Specific version
- `hyperb1iss/git-iris:main` - Latest build from the main branch

## 🚀 Basic Usage

Here's how to run Git-Iris with Docker:

```bash
# View help
docker run --rm hyperb1iss/git-iris

# Generate a commit message (mount current directory)
docker run --rm --user $(id -u):$(id -g) -v "$(pwd):/git-repo" hyperb1iss/git-iris gen
```

## 🔑 Configuration with Environment Variables

You can configure Git-Iris using environment variables:

```bash
docker run --rm --user $(id -u):$(id -g) -v "$(pwd):/git-repo" \
  -e GITIRIS_PROVIDER="openai" \
  -e GITIRIS_API_KEY="your-api-key" \
  -e GITIRIS_MODEL="gpt-4o" \
  -e GITIRIS_PRESET="conventional" \
  -e GITIRIS_GITMOJI="true" \
  -e GIT_USER_NAME="Your Name" \
  -e GIT_USER_EMAIL="your.email@example.com" \
  hyperb1iss/git-iris gen
```

### Environment Variables

| Variable                   | Description                                                                                       |
| -------------------------- | ------------------------------------------------------------------------------------------------- |
| `GITIRIS_PROVIDER`         | LLM provider name (e.g., openai, anthropic, ollama)                                               |
| `GITIRIS_API_KEY`          | API key for the provider                                                                          |
| `GITIRIS_MODEL`            | Model to use (provider-specific)                                                                  |
| `GITIRIS_TOKEN_LIMIT`      | Token limit for the specified provider                                                            |
| `GITIRIS_DEFAULT_PROVIDER` | Default LLM provider to use                                                                       |
| `GITIRIS_PRESET`           | Instruction preset name                                                                           |
| `GITIRIS_GITMOJI`          | Enable/disable Gitmoji (true/false)                                                               |
| `GITIRIS_INSTRUCTIONS`     | Custom instructions for AI responses                                                              |
| `GITIRIS_PARAMS`           | Additional parameters as comma-separated key=value pairs (e.g., "temperature=0.7,max_tokens=150") |
| `GITIRIS_FORCE_CONFIG`     | Force config update even if config file exists (true/false)                                       |
| `GIT_USER_NAME`            | Git user name                                                                                     |
| `GIT_USER_EMAIL`           | Git user email                                                                                    |

Example with advanced configuration:

```bash
docker run --rm --user $(id -u):$(id -g) -v "$(pwd):/git-repo" \
  -e GITIRIS_PROVIDER="anthropic" \
  -e GITIRIS_API_KEY="your-api-key" \
  -e GITIRIS_MODEL="claude-3-7-sonnet-20250219" \
  -e GITIRIS_TOKEN_LIMIT="200000" \
  -e GITIRIS_PRESET="detailed" \
  -e GITIRIS_INSTRUCTIONS="Always include the ticket number and highlight performance impacts" \
  -e GITIRIS_PARAMS="temperature=0.7,max_tokens=4000" \
  -e GITIRIS_GITMOJI="true" \
  hyperb1iss/git-iris gen
```

### Handling Repository Permissions

When working with Git repositories mounted from your host system, you'll typically need to use the `--user` flag to ensure proper permissions:

```bash
# Mount your repository with the correct user permissions
docker run --rm --user $(id -u):$(id -g) -v "$(pwd):/git-repo" hyperb1iss/git-iris gen
```

This passes your user and group IDs to the container, allowing Git-Iris to:

1. Read your repository's files with the correct permissions
2. Write commits as your user (rather than as root)
3. Avoid "repository not owned by current user" errors from Git

Without the `--user` flag, you may encounter permission issues when:

- Working with repositories that have strict ownership checks
- Running git commands that modify the repository
- Accessing credential stores or config files

For read-only operations that don't interact with the repository (like `--help` or `--version`), the `--user` flag is not necessary.

## 🔄 Persistent Configuration

To persist your configuration between runs, mount a volume to `/root/.config/git-iris`:

```bash
docker run --rm --user $(id -u):$(id -g) -v "$(pwd):/git-repo" \
  -v git-iris-config:/root/.config/git-iris \
  hyperb1iss/git-iris config --provider openai --api-key your-api-key
```

Now your configuration will be saved for future runs:

```bash
docker run --rm --user $(id -u):$(id -g) -v "$(pwd):/git-repo" \
  -v git-iris-config:/root/.config/git-iris \
  hyperb1iss/git-iris gen
```

## 🛠️ CI/CD Integration

Git-Iris works well in CI/CD pipelines. Here's an example for GitHub Actions:

```yaml
name: Generate Release Notes

on:
  push:
    tags:
      - "v*"

jobs:
  release-notes:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Generate Release Notes
        env:
          GITIRIS_PROVIDER: openai
          GITIRIS_API_KEY: ${{ secrets.OPENAI_API_KEY }}
        run: |
          docker run --rm --user $(id -u):$(id -g) -v "$(pwd):/git-repo" \
            -e GITIRIS_PROVIDER -e GITIRIS_API_KEY \
            hyperb1iss/git-iris release-notes \
            --from $(git describe --tags --abbrev=0 $(git rev-list --tags --skip=1 --max-count=1)) \
            --to $(git describe --tags --abbrev=0) \
            --print > RELEASE_NOTES.md

      - name: Create Release
        uses: softprops/action-gh-release@v2
        with:
          body_path: RELEASE_NOTES.md
          files: |
            # Add your release files here
```

## CI Pipeline Examples

### GitHub Actions - Automatic Changelog in PRs

```yaml
name: PR Changelog

on:
  pull_request:
    types: [opened, synchronize]
    branches: [main, master]

jobs:
  generate-changelog:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      
      - name: Generate PR Changelog
        env:
          GITIRIS_PROVIDER: openai
          GITIRIS_API_KEY: ${{ secrets.OPENAI_API_KEY }}
          GITIRIS_PRESET: detailed
        run: |
          docker run --rm -v "$(pwd):/git-repo" \
            -e GITIRIS_PROVIDER -e GITIRIS_API_KEY -e GITIRIS_PRESET \
            hyperb1iss/git-iris changelog \
            --from ${{ github.event.pull_request.base.sha }} \
            --to ${{ github.event.pull_request.head.sha }} \
            --print > PR_CHANGELOG.md
      
      - name: Comment on PR
        uses: actions/github-script@v6
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
          script: |
            const fs = require('fs');
            const changelog = fs.readFileSync('PR_CHANGELOG.md', 'utf8');
            const body = `## AI-Generated Changelog\n\n${changelog}`;
            
            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: body
            });
```

### GitLab CI - Automated Code Reviews

```yaml
code-review:
  image: docker:20.10.16
  stage: test
  services:
    - docker:20.10.16-dind
  variables:
    GITIRIS_PROVIDER: anthropic
    GITIRIS_PRESET: security
  script:
    - docker run --rm -v "$CI_PROJECT_DIR:/git-repo" 
      -e GITIRIS_PROVIDER -e GITIRIS_API_KEY
      -e GITIRIS_PRESET
      hyperb1iss/git-iris review
      --from $CI_MERGE_REQUEST_DIFF_BASE_SHA
      --to $CI_COMMIT_SHA
      --print > code_review.md
  artifacts:
    paths:
      - code_review.md
    expire_in: 1 week
  only:
    - merge_requests
```

## 🔧 Building Custom Images

You can build a custom image with your preferred configuration:

```dockerfile
FROM hyperb1iss/git-iris:latest

# Set default configuration
ENV GITIRIS_PROVIDER="anthropic"
ENV GITIRIS_MODEL="claude-3-7-sonnet-20250219"
ENV GITIRIS_PRESET="conventional"
ENV GITIRIS_GITMOJI="true"

# Add your custom entrypoint or scripts if needed
```

Build it:

```bash
docker build -t my-custom-git-iris .
```

## 💡 Tips and Tricks

- For interactive commands, be sure to include `-it` flags: `docker run -it --rm hyperb1iss/git-iris gen`
- To use a specific version: `docker run --rm hyperb1iss/git-iris:1.0.1 gen`
- If you're working with a private repository, make sure to mount your SSH keys or configure Git credentials
- Add an alias to your shell for convenience:
  ```bash
  alias git-iris='docker run -it --rm -v "$(pwd):/git-repo" -v git-iris-config:/root/.config/git-iris hyperb1iss/git-iris'
  ```

## 🐞 Troubleshooting

- **Error: "Git repository not found"**: Make sure you're mounting your Git repository to `/git-repo`
- **Permission issues**: The container runs as root by default. If needed, adjust volume permissions with the `--user` flag.
- **Git config permissions error**: If you get errors like "could not lock config file", try adding these environment variables:
  ```bash
  -e GIT_CONFIG_NOSYSTEM="1" -e HOME="/tmp"
  ```
- **API Key not working**: Verify your API key is correctly passed via environment variable.
- **Interactive features not working**: Ensure you're using the `-it` flags when running Docker.

## 🛠️ Building the Docker Image

If you want to build the Docker image yourself, you can use the included build script:

```bash
# Build with the default "dev" tag
./docker/build.sh

# Build with a custom tag
./docker/build.sh mytag
```

The build script will:

1. Build the Docker image
2. Offer to run tests on the built image
3. Provide instructions for running and pushing the image

## 🧪 Testing the Docker Image

You can test a built or pulled image with the test script:

```bash
# Test the latest image against the current directory
./docker/test-image.sh latest

# Test a specific tag
./docker/test-image.sh 1.0.1

# Test against a specific repository path
./docker/test-image.sh latest /path/to/your/repo

# For more reliable tests, export your OpenAI API key first:
export OPENAI_API_KEY=your-api-key
./docker/test-image.sh latest
```

The test script runs several basic tests to verify that the image works correctly:

- Checks that the help command works
- Verifies the version command
- Tests with your repository (or the current directory by default)
- Validates environment variable handling

For the best test reliability, provide an `OPENAI_API_KEY` environment variable. The test script automatically detects and uses this key, allowing tests to use the real OpenAI provider instead of the test provider, which may fail in production builds.

If the `OPENAI_API_KEY` is not available, the script will fall back to using a test provider and show a warning message.

> **Note:** The auto-commit test may show permission warnings when running in Docker. This is expected behavior due to Docker volume mounting and permission limitations. The test will check for successful commit message generation but may not be able to actually create a commit within the container.

## 🔄 Continuous Integration

The Git-Iris project uses GitHub Actions to automatically build and publish Docker images for each release. The workflow:

1. Builds on new tags and the main branch
2. Tags images appropriately (version number, latest, etc.)
3. Runs tests to ensure functionality
4. Pushes to Docker Hub
