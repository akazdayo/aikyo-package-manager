# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is APM (Aikyo Package Manager), a Rust-based plugin management system that helps manage Git repositories as plugins. The tool automatically clones Git repositories into a `tools/` directory and tracks them via a `apm.toml` configuration file.

## Commands

### Build and Development
- `cargo build` - Build the project
- `cargo run -- <subcommand>` - Run the APM tool with specified subcommand
- `cargo test` - Run tests
- `cargo check` - Quick syntax and type checking

### APM Usage
- `cargo run -- add <git-url>` - Add a plugin by Git URL
- `cargo run -- sync` - Sync plugins (clone missing repositories)
- `cargo run -- init` - Initialize project configuration
- `cargo run -- remove` - Remove plugin (not yet implemented)

## Architecture

### Core Modules

- **main.rs**: Entry point with CLI parsing using clap, defines the main subcommands (Add, Remove, Sync, Init)
- **manager.rs**: Configuration management with `Config` and `Project` structs, handles `apm.toml` file I/O
- **sync.rs**: Plugin synchronization logic, parses Git URLs and manages repository cloning

### Key Components

1. **Config System** (`manager.rs:6-63`):
   - `Project` struct contains plugins list and tools directory path
   - Auto-creates `apm.toml` if it doesn't exist with default `./tools` directory
   - Handles plugin addition to configuration

2. **Sync System** (`sync.rs:9-86`):
   - Parses Git repository names from URLs using regex
   - Compares existing directories with configured plugins
   - Clones missing repositories using `git clone`

### Configuration File Format

The `apm.toml` file structure:
```toml
[project]
plugins = ["https://github.com/user/repo.git"]
tools_dir = "./tools"
```

### Important Implementation Details

- Uses regex pattern `/([^/]+)\.git$` to extract repository names from Git URLs
- Automatically creates the tools directory if it doesn't exist
- Plugin synchronization only clones missing repositories, doesn't update existing ones
- The `append_plugin` method in `manager.rs:58` has a bug - it saves to `tools_dir` instead of `apm.toml`