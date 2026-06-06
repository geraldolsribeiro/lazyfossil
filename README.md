# lazyfossil

[![Crates.io](https://img.shields.io/crates/v/lazyfossil)](https://crates.io/crates/lazyfossil)
[![Multiplatform Release](https://github.com/geraldolsribeiro/lazyfossil/actions/workflows/release.yml/badge.svg)](https://github.com/geraldolsribeiro/lazyfossil/actions/workflows/release.yml)
[![GitHub stars](https://img.shields.io/github/stars/geraldolsribeiro/lazyfossil)](https://github.com/geraldolsribeiro/lazyfossil/stargazers)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

![lazyfossil logo](doc/images/lazyfossil_logo.png)

A lazygit-inspired terminal UI for Fossil SCM.

## Project goals

- Fast terminal workflow for Fossil checkouts
- Working tree and history browsing in one UI
- Commit subsets of files without a staging area
- Small, practical MVP first; polish later

## Features

### Repository browsing
- Fossil checkout detection
- Working tree file list
- Diff / details pane
- Timeline / history view
- File-history timeline for selected path
- Hidden-file listing (`extras --dotfiles`)
- Keyboard and mouse navigation

### Commit and sync flow
- Temporary commit selection with `Space`
- Commit selected files, current file, or all files
- Extra-file commit support
- Ignore-file editing via `.fossil-settings/ignore-glob`
- Sync with remote via `p` / `P`
- Inline commit and ignore prompts
- Command logging to `fossil-debug.log`
- Binary file handling before commit via `binary-glob`

### Preview and UI polish
- Binary preview fallback with friendly notice
- Tab-expanded text preview (e.g. Makefiles)
- Footer/input UX improvements
- Reusable ASCII logo text asset

### Project and release polish
- Project logo asset and README branding
- GitHub Actions release workflow fix for Windows executable naming
- README credits section
- Cargo package/versioning/release housekeeping
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
- Working tree MVP
- History timeline basics
- Temporary selection-based commit flow
- Inline commit message prompt
- Ignore-file support
- Sync support
- Binary preview fallback

### Next
- Commit details and file history in the history pane
- Footer/status layout polish
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
Current version: `0.3.2`.

## Credits

### [pi.dev](https://pi.dev)
Pi provides the agent harness used to shape and iterate on this project. Its tooling made it possible to refine the TUI, validate changes, and keep the implementation moving quickly.

### [crates.io/crates/lazyfossil](https://crates.io/crates/lazyfossil)
The crate listing is the distribution point for the Rust application, making the project available to the wider Rust ecosystem and simplifying installation and release management.

### [emojicombos.com/lazyfossil](https://emojicombos.com/lazyfossil)
This source provided the project logo artwork used in the README and assets, helping give lazyfossil a recognizable visual identity.
