# lazyfossil

[![Crates.io](https://img.shields.io/crates/v/lazyfossil)](https://crates.io/crates/lazyfossil)
[![Multiplatform Release](https://github.com/geraldolsribeiro/lazyfossil/actions/workflows/release.yml/badge.svg)](https://github.com/geraldolsribeiro/lazyfossil/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![GitHub stars](https://img.shields.io/github/stars/geraldolsribeiro/lazyfossil)](https://github.com/geraldolsribeiro/lazyfossil/stargazers)

![lazyfossil logo](doc/images/lazyfossil_logo.png)

A lazygit-inspired terminal UI for Fossil SCM.

## Project goals

- Provide a fast terminal workflow for Fossil checkouts
- Combine working-tree and history browsing in a single UI
- Commit subsets of files without a staging area
- Start with a small, practical MVP and refine it over time

## Installation

### From crates.io

**lazyfossil** is written in Rust and can be installed with `cargo`:

```bash
cargo install lazyfossil
```

### Pre-built binaries

Download pre-built binaries from the GitHub releases page:

<https://github.com/geraldolsribeiro/lazyfossil/releases/>

Available builds:

- Linux
- macOS
- Windows

## Source code

The **lazyfossil** source code is maintained in **Fossil SCM** and mirrored to **GitHub** and **crates.io**.
A public Fossil repository will be available soon, likely on <https://chiselapp.com/>.

## Screenshots

**lazyfossil** is currently in alpha, but it is already usable. There is still plenty to polish.

![Screenshot 01](doc/images/screenshot_01.png)

![Screenshot 02](doc/images/screenshot_01.png)

## Features

### Repository browsing
- Fossil checkout detection
- Working-tree file list
- Diff/details pane
- Timeline/history view
- File-history timeline for the selected path
- Hidden-file listing (`extras --dotfiles`)
- Keyboard and mouse navigation

### Commit and sync flow
- Temporary commit selection with `Space`
- Commit selected files, the current file, or all files
- Extra-file commit support
- Ignore-file editing via `.fossil-settings/ignore-glob`
- Sync with the remote via `p` / `P`
- Inline commit and ignore prompts
- Command logging to `fossil-debug.log`
- Binary-file handling before commit via `binary-glob`

### Preview and UI polish
- Binary preview fallback with a friendly notice
- Tab-expanded text previews (for example, Makefiles)
- Footer and input UX improvements
- Reusable ASCII logo text asset

### Project and release polish
- Project logo asset and README branding
- GitHub Actions release workflow fix for Windows executable naming
- README credits section
- Cargo package, versioning, and release housekeeping
- Additional test coverage for parsing, selection, and prompts

## Commit flow

Fossil does not use a staging area.
Instead, lazyfossil builds commit commands like:

```bash
fossil commit -m "commit message" file1 file2 file3
```

Extra files are added automatically before commit when needed.

Binary files are handled by setting:

```bash
fossil settings binary-glob "*.png,*.jpg,*.jpeg,*.gif,*.ico"
```

## Roadmap

### Done
- Working-tree MVP
- History timeline basics
- Temporary selection-based commit flow
- Inline commit message prompt
- Ignore-file support
- Sync support
- Binary preview fallback

### Next
- Commit details and file history in the history pane
- Footer and status layout polish
- Better mouse interactions and scrolling

## Build

```bash
cargo build --release
```

## Run

```bash
cargo run
```

## Versioning

This project follows semantic versioning: `MAJOR.MINOR.PATCH`.
Current version: `0.3.3`.

## Credits

### [pi.dev](https://pi.dev)
Pi provides the agent harness used to develop and refine this project. Its tooling made it easier to iterate quickly, validate changes, and improve the TUI with confidence.

### [crates.io/crates/lazyfossil](https://crates.io/crates/lazyfossil)
The crates.io listing is the distribution channel for the Rust application, helping make lazyfossil available to the broader Rust ecosystem and simplifying installation and release management.

### [emojicombos.com/lazyfossil](https://emojicombos.com/lazyfossil)
This source provided the project logo artwork used in the README and assets, giving lazyfossil a recognizable visual identity.
