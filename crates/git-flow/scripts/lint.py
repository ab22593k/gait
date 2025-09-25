#!/usr/bin/env python3

"""
RustLint 🦀 - Elevating Rust Code Quality with Style

A smart lint tool that maintains the highest standards
for your Rust codebase.

Features:
- Code formatting (cargo fmt) - Ensuring style consistency
- Linting (cargo clippy) - Proactive error detection
- Automatic fixes (cargo fix) - Streamlined issue resolution
- Git-aware scanning - Focused quality analysis
- Clear, elegant output - Because elegance matters

Usage:
    lint.py                  # Full lint checks (cargo clippy)
    lint.py --format         # Format code (cargo fmt)
    lint.py path/to/project  # Lint specific cargo projects
    lint.py -g               # Process git-modified projects
    lint.py --fix            # Attempt automatic fixes (cargo fix)
"""

import argparse
import os
import subprocess
import sys
from abc import ABC
from dataclasses import dataclass

# ANSI color codes
GREEN = "\033[92m"
RED = "\033[91m"
BLUE = "\033[94m"
RESET = "\033[0m"

# Default directory to run the commands on (typically the project root)
DEFAULT_CARGO_DIR = "."


# pylint: disable=too-few-public-methods
@dataclass
class ToolResult:
    """Result of running a tool."""

    name: str
    success: bool
    stdout: str
    stderr: str


class BaseRustTool(ABC):
    """Base class for Rust tools like format, linter, and fixer."""

    def __init__(self, name: str, command_factory):
        """Initialize the BaseRustTool with a name and a command factory function."""
        self.name = name
        self.command_factory = command_factory

    def run(self, directory: str) -> ToolResult:
        """Run the tool command in the given directory.

        If the directory is not a valid Cargo project (i.e., Cargo.toml is missing),
        this method skips execution and returns a success result indicating the skip.
        """
        if not os.path.exists(os.path.join(directory, "Cargo.toml")):
            print(
                f"{BLUE}Skipping {directory}: "
                f"Cargo.toml not found (not a Rust project).{RESET}"
            )
            return ToolResult(self.name, True, f"Skipped {directory}", "")
        command = self.command_factory(directory)
        process = subprocess.run(
            command,
            cwd=directory,
            capture_output=True,
            text=True,
            check=False,
        )
        return ToolResult(
            name=self.name,
            success=process.returncode == 0,
            stdout=process.stdout,
            stderr=process.stderr,
        )


class RustToolRunner:
    """Manages and runs Rust tools (clippy, fmt, fix) across cargo projects."""

    def __init__(self) -> None:
        """Initialize with the available tools for linting, formatting, and fixing."""
        self.linters = {
            "clippy": BaseRustTool(
                "clippy", lambda d: ["cargo", "clippy", "--", "-D", "warnings"]
            ),
        }
        self.formatters = {
            "fmt": BaseRustTool("fmt", lambda d: ["cargo", "fmt"]),
        }
        self.fixers = {
            "fix": BaseRustTool("fix", lambda d: ["cargo", "fix"]),
        }
        self.unsafe_fixers = {
            "fix": BaseRustTool(
                "fix", lambda d: ["cargo", "fix", "--allow-dirty", "--allow-staged"]
            ),
        }

    def get_tools_to_run(
        self,
        tools: dict,
        selected_tools: list[str] | None = None,
        _is_git_modified: bool = False,
    ) -> dict:
        """Filter and return the rust tools to run based on the selected tools list.

        If selected_tools is provided, only return tools whose names are in selected_tools.
        Otherwise, return all tools.
        """
        if selected_tools:
            return {
                name: tool for name, tool in tools.items() if name in selected_tools
            }
        return tools.copy()

    def handle_result(self, result: ToolResult, directory: str) -> bool:
        """Handle and display the tool's result for a given directory.

        Returns True if the tool failed, otherwise False.
        """
        if not result.success:
            print(f"{RED}❌ {result.name.capitalize()} issues in {directory}:{RESET}")
            print(result.stdout)
            if result.stderr:
                print(result.stderr)
            return True
        print(
            f"{GREEN}✅ {result.name.capitalize()} checks passed in {directory}.{RESET}"
        )
        return False

    def run_on_dirs(self, tools: dict, dirs: list[str]) -> list[str]:
        """Run each tool on the provided directories and return the names of any that fail."""
        failures = []
        for tool in tools.values():
            tool_failed = False
            for d in dirs:
                result = tool.run(d)
                if self.handle_result(result, d):
                    tool_failed = True
            if tool_failed:
                failures.append(tool.name)
        return failures

    # pylint: disable=too-many-arguments,too-many-positional-arguments
    def run(
        self,
        target_dirs: list[str] | None = None,
        selected_tools: list[str] | None = None,
        is_formatting: bool = False,
        is_fixing: bool = False,
        is_unsafe_fixing: bool = False,
        is_git_modified: bool = False,
    ) -> None:
        """Run the selected Rust tool(s) on the target directories.

        Depending on the provided flags, runs formatters, fixers, or linters.
        Exits with code 1 if any tool fails, otherwise exits with code 0.
        """
        if is_formatting:
            print(f"{BLUE}🎨 Running formatter (cargo fmt)...{RESET}")
            tools_to_run = self.get_tools_to_run(
                self.formatters, selected_tools, is_git_modified
            )
        elif is_fixing:
            print(f"{BLUE}🔧 Running fixer (cargo fix)...{RESET}")
            if is_unsafe_fixing:
                tools_to_run = self.get_tools_to_run(
                    self.unsafe_fixers, selected_tools, is_git_modified
                )
            else:
                tools_to_run = self.get_tools_to_run(
                    self.fixers, selected_tools, is_git_modified
                )
        else:
            print(f"{BLUE}🔎 Running linter (cargo clippy)...{RESET}")
            tools_to_run = self.get_tools_to_run(
                self.linters, selected_tools, is_git_modified
            )

        paths = target_dirs if target_dirs else [DEFAULT_CARGO_DIR]
        failures = self.run_on_dirs(tools_to_run, paths)

        if failures:
            print(
                f"\n{RED}💥 The following tools failed: "
                f"{', '.join(failures)}{RESET}"
            )
            sys.exit(1)

        action = (
            "formatting"
            if is_formatting
            else "fixes"
            if is_fixing
            else "linting checks"
        )
        print(f"\n{GREEN}🎉 All {action} completed successfully!{RESET}")
        sys.exit(0)


def find_cargo_manifest_dir(path: str) -> str | None:
    """
    Find the closest directory containing Cargo.toml starting from the file's directory.
    """
    curr_dir = os.path.abspath(os.path.dirname(path))
    while True:
        if os.path.exists(os.path.join(curr_dir, "Cargo.toml")):
            return curr_dir
        parent = os.path.abspath(os.path.join(curr_dir, os.pardir))
        if curr_dir == parent:
            break
        curr_dir = parent
    return None


def get_modified_files() -> list[str]:
    """Get list of modified files (both staged and unstaged) from git."""
    staged = subprocess.run(
        ["git", "diff", "--cached", "--name-only"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout.splitlines()

    unstaged = subprocess.run(
        ["git", "diff", "--name-only"],
        capture_output=True,
        text=True,
        check=False,
    ).stdout.splitlines()

    # Combine and remove duplicates while preserving order
    modified = list(dict.fromkeys(staged + unstaged))
    modified = [f for f in modified if os.path.exists(f)]
    if not modified:
        print(f"{BLUE}No modified files found.{RESET}")
        sys.exit(0)
    return modified


def get_modified_dirs() -> list[str]:
    """
    Determine cargo project directories from git modified files by locating the Cargo.toml.
    """
    modified_files = get_modified_files()
    dirs = []
    for file in modified_files:
        manifest_dir = find_cargo_manifest_dir(file)
        if manifest_dir and manifest_dir not in dirs:
            dirs.append(manifest_dir)
    if not dirs:
        print(f"{BLUE}No modified cargo projects found.{RESET}")
        sys.exit(0)
    return dirs


def main() -> None:
    """Main entry point for the Rust lint command."""
    runner = RustToolRunner()

    parser = argparse.ArgumentParser(
        description="Run Rust linting and formatting checks in cargo projects."
    )
    parser.add_argument(
        "paths",
        nargs="*",
        help="Paths to cargo projects (directories with Cargo.toml).",
    )
    parser.add_argument(
        "--linters",
        nargs="+",
        choices=list(runner.linters.keys()),
        help="Specific linters to run. If not provided, default is 'clippy'.",
    )
    parser.add_argument(
        "--format",
        action="store_true",
        help="Run formatters (cargo fmt).",
    )
    parser.add_argument(
        "--formatters",
        nargs="+",
        choices=list(runner.formatters.keys()),
        help="Specific formatters to run. If not provided, all formatters will be run.",
    )
    parser.add_argument(
        "--fix",
        action="store_true",
        help="Run cargo fix to automatically fix issues.",
    )
    parser.add_argument(
        "--unsafe-fixes",
        action="store_true",
        help="Run cargo fix with --allow-dirty and --allow-staged.",
    )
    parser.add_argument(
        "--git-modified",
        "-g",
        action="store_true",
        help="Run on git modified files (determine cargo project roots from modified files).",
    )
    args = parser.parse_args()

    if args.git_modified:
        target_paths = get_modified_dirs()
    elif args.paths:
        target_paths = args.paths
    else:
        target_paths = [DEFAULT_CARGO_DIR]

    # Extract selected_tools to split the long parameter line
    selected_tools = args.formatters if args.format else args.linters
    runner.run(
        target_dirs=target_paths,
        selected_tools=selected_tools,
        is_formatting=args.format,
        is_fixing=args.fix,
        is_unsafe_fixing=args.unsafe_fixes,
        is_git_modified=args.git_modified,
    )


if __name__ == "__main__":
    main()
