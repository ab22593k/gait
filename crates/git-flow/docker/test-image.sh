#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🧪 Testing Git-Iris Docker Image${NC}"

# Get the tag/version to test
TAG=${1:-"latest"}
# Get the repository path to test against (defaults to current directory)
REPO_PATH=${2:-$(pwd)}
IMAGE="hyperb1iss/git-iris:${TAG}"
ERROR_COUNT=0

# Create a test repository if needed
if [ ! -d "${REPO_PATH}/.git" ]; then
  echo -e "${YELLOW}No git repo found at $REPO_PATH - creating a temporary test repository${NC}"
  TEST_REPO=$(mktemp -d)
  echo -e "${BLUE}Created test repository at $TEST_REPO${NC}"

  # Initialize git repository
  (
    cd "$TEST_REPO" &&
      git init &&
      git config user.name "Git-Iris Test User" &&
      git config user.email "test@example.com" &&
      echo "# Test Repository" >README.md &&
      git add README.md &&
      git commit -m "Initial commit" &&
      echo 'function test() { return "Hello, world!"; }' >test.js &&
      git add test.js
  )

  # Use the test repo for testing
  REPO_PATH="$TEST_REPO"

  # Make sure to clean up the test repo on exit
  cleanup_test_repo() {
    echo -e "\n${YELLOW}🧹 Cleaning up test repository...${NC}"
    rm -rf "$TEST_REPO"

    if [ $ERROR_COUNT -gt 0 ]; then
      echo -e "\n${RED}❌ Failed $ERROR_COUNT tests${NC}"
      exit 1
    else
      echo -e "\n${GREEN}🎉 All tests passed!${NC}"
    fi
  }

  trap cleanup_test_repo EXIT INT TERM
else
  # Regular cleanup function for non-temp repos
  cleanup() {
    if [ $ERROR_COUNT -gt 0 ]; then
      echo -e "\n${RED}❌ Failed $ERROR_COUNT tests${NC}"
      exit 1
    else
      echo -e "\n${GREEN}🎉 All tests passed!${NC}"
    fi
  }

  trap cleanup EXIT INT TERM
fi

# Check if OpenAI API key is available in the environment
if [ -n "$OPENAI_API_KEY" ]; then
  PROVIDER="openai"
  API_KEY="$OPENAI_API_KEY"
  MODEL="gpt-3.5-turbo" # Using a smaller, faster model for tests
  REAL_API_KEY=true
  echo -e "${GREEN}✨ Using OpenAI for provider tests (API key found in environment)${NC}"
else
  # Use a valid provider name but mark that we don't have a real key
  PROVIDER="openai"
  API_KEY="sk-xxxx" # Placeholder that won't work for actual API calls
  MODEL="gpt-3.5-turbo"
  REAL_API_KEY=false
  echo -e "${YELLOW}⚠️ No OPENAI_API_KEY found in environment${NC}"
  echo -e "${YELLOW}⚠️ Basic image tests will run, but API-dependent tests will be skipped${NC}"
  echo -e "${YELLOW}⚠️ For full testing, export OPENAI_API_KEY=your-api-key${NC}"
fi

# Function to run tests
run_test() {
  local test_name=$1
  local command=$2
  local expected_status=${3:-0}
  local expected_output_pattern=$4

  # If we have an OPENAI_API_KEY, pass it to the container
  if [ -n "$OPENAI_API_KEY" ] && [[ "$command" == *"docker run"* ]] && [[ "$command" != *"OPENAI_API_KEY"* ]]; then
    command="${command/docker run/docker run -e OPENAI_API_KEY=\"$OPENAI_API_KEY\"}"
  fi

  echo -e "\n${YELLOW}Running test: ${test_name}${NC}"
  echo -e "${BLUE}Command: ${command}${NC}"

  # Run the command and capture output
  echo -e "${BLUE}Output:${NC}"
  # Use a temporary file to capture output
  temp_output_file=$(mktemp)
  set +e # Disable 'exit on error' temporarily
  eval "$command" >"$temp_output_file" 2>&1
  status=$?
  set -e # Re-enable 'exit on error'

  # Display the output
  cat "$temp_output_file"

  # Store output for pattern matching
  output=$(<"$temp_output_file")
  rm "$temp_output_file"

  # Check status
  if [ $status -ne $expected_status ]; then
    echo -e "${RED}❌ Test failed: Expected status $expected_status, got $status${NC}"
    ERROR_COUNT=$((ERROR_COUNT + 1))
    return 1
  fi

  # Check output pattern if specified
  if [ -n "$expected_output_pattern" ] && ! echo "$output" | grep -E "$expected_output_pattern"; then
    echo -e "${RED}❌ Test failed: Output does not contain expected pattern: $expected_output_pattern${NC}"
    ERROR_COUNT=$((ERROR_COUNT + 1))
    return 1
  fi

  echo -e "${GREEN}✅ Test passed: $test_name${NC}"
  return 0
}

# Show repository information
echo -e "\n${YELLOW}🐳 Testing with repository: ${REPO_PATH}${NC}"
if [ -d "${REPO_PATH}/.git" ]; then
  echo -e "${GREEN}✓ Valid Git repository found${NC}"
  (cd "$REPO_PATH" && git config --get user.name && git config --get user.email) ||
    echo -e "${YELLOW}⚠️ Git user not configured in the repository${NC}"
else
  echo -e "${YELLOW}⚠️ Not a Git repository. Basic tests will still run.${NC}"
fi

echo -e "\n${YELLOW}📋 Test 1: Help command${NC}"
run_test "Help command" "docker run --rm $IMAGE --help"

echo -e "\n${YELLOW}📋 Test 2: Version command${NC}"
run_test "Version command" "docker run --rm $IMAGE --version"

echo -e "\n${YELLOW}📋 Test 3: List presets command${NC}"
run_test "List presets command" "docker run --rm $IMAGE list-presets"

# Skip environment variable tests that use the provider if we don't have a real key
if [ "$REAL_API_KEY" = true ]; then
  echo -e "\n${YELLOW}📋 Test 4: Basic environment variables${NC}"
  run_test "Basic environment variables" "docker run --rm \
    -e GIT_USER_NAME=\"Docker Test User\" \
    -e GIT_USER_EMAIL=\"docker@test.com\" \
    -e GITIRIS_PROVIDER=\"$PROVIDER\" \
    -e GITIRIS_API_KEY=\"$API_KEY\" \
    $IMAGE --version"

  echo -e "\n${YELLOW}📋 Test 5: Advanced environment variables${NC}"
  run_test "Advanced environment variables" "docker run --rm \
    -e GITIRIS_PROVIDER=\"$PROVIDER\" \
    -e GITIRIS_MODEL=\"$MODEL\" \
    -e GITIRIS_TOKEN_LIMIT=\"10000\" \
    -e GITIRIS_PRESET=\"detailed\" \
    -e GITIRIS_INSTRUCTIONS=\"Test instructions\" \
    -e GITIRIS_GITMOJI=\"true\" \
    -e GITIRIS_PARAMS=\"temperature=0.7,max_tokens=500\" \
    -e GITIRIS_FORCE_CONFIG=\"true\" \
    $IMAGE --version"
else
  echo -e "\n${YELLOW}📋 Test 4: Basic environment variables (without provider)${NC}"
  run_test "Basic environment variables (without provider)" "docker run --rm \
    -e GIT_USER_NAME=\"Docker Test User\" \
    -e GIT_USER_EMAIL=\"docker@test.com\" \
    $IMAGE --version"

  echo -e "\n${YELLOW}📋 Test 5: Advanced environment variables (without provider)${NC}"
  run_test "Advanced environment variables (without provider)" "docker run --rm \
    -e GITIRIS_PRESET=\"detailed\" \
    -e GITIRIS_INSTRUCTIONS=\"Test instructions\" \
    -e GITIRIS_GITMOJI=\"true\" \
    -e GITIRIS_PARAMS=\"temperature=0.7,max_tokens=500\" \
    -e GITIRIS_FORCE_CONFIG=\"true\" \
    $IMAGE --version"

  echo -e "\n${YELLOW}⚠️ Skipping provider-specific environment variable tests (no real API key)${NC}"
fi

echo -e "\n${YELLOW}📋 Test 6: Multiple subcommands${NC}"
run_test "Multiple subcommands" "docker run --rm $IMAGE config --help"

# Only run repository-specific tests if we have a real API key
if [ "$REAL_API_KEY" = true ]; then
  # Run git-iris with the mounted repository in print mode
  echo -e "\n${YELLOW}📋 Test 7: Repository commands${NC}"

  # Run git-iris in print-only mode
  echo -e "${BLUE}Running Git-Iris in print-only mode with the repository...${NC}"
  run_test "Print commit message" "docker run --rm \
    --user $(id -u):$(id -g) \
    -v \"${REPO_PATH}:/git-repo\" \
    -e GIT_USER_NAME=\"Docker Test User\" \
    -e GIT_USER_EMAIL=\"docker@test.com\" \
    -e GITIRIS_PROVIDER=\"$PROVIDER\" \
    -e GITIRIS_API_KEY=\"$API_KEY\" \
    -e GITIRIS_MODEL=\"$MODEL\" \
    -e GITIRIS_FORCE_CONFIG=\"true\" \
    -e GIT_CONFIG_NOSYSTEM=\"1\" \
    -e HOME=\"/tmp\" \
    -e GITIRIS_INSTRUCTIONS=\"IMPORTANT: Include the ticket number IRIS-123 in your commit message.\" \
    $IMAGE gen --print" "0" "IRIS-123"

  # Run auto-commit test if we're not in CI
  if [ -z "$CI" ]; then
    echo -e "${BLUE}Testing auto-commit mode...${NC}"
    run_test "Auto-commit" "docker run --rm \
      --user $(id -u):$(id -g) \
      -v \"${REPO_PATH}:/git-repo\" \
      -e GIT_USER_NAME=\"Docker Test User\" \
      -e GIT_USER_EMAIL=\"docker@test.com\" \
      -e GITIRIS_PROVIDER=\"$PROVIDER\" \
      -e GITIRIS_API_KEY=\"$API_KEY\" \
      -e GITIRIS_MODEL=\"$MODEL\" \
      -e GITIRIS_FORCE_CONFIG=\"true\" \
      -e GIT_CONFIG_NOSYSTEM=\"1\" \
      -e HOME=\"/tmp\" \
      -e GITIRIS_INSTRUCTIONS=\"IMPORTANT: Include the ticket number IRIS-123 in your commit message.\" \
      $IMAGE gen -a" "0" "IRIS-123"

    # Verify the commit was made
    echo -e "${BLUE}Verifying commit:${NC}"
    (cd "$REPO_PATH" && git log -1 --pretty=full) || echo -e "${YELLOW}⚠️ Could not read commit log${NC}"
  else
    echo -e "${YELLOW}⚠️ Skipping auto-commit test in CI environment${NC}"
  fi
else
  echo -e "\n${YELLOW}⚠️ Skipping repository-specific tests (no real API key)${NC}"
  echo -e "${YELLOW}   To run all tests, provide your OpenAI API key:${NC}"
  echo -e "${BLUE}   export OPENAI_API_KEY=your-api-key${NC}"
fi
