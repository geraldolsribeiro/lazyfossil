# lazyfossil

A lazygit-inspired terminal UI for Fossil SCM.

## Versioning

This project follows semantic versioning: `MAJOR.MINOR.PATCH`.
Current version: `0.2.1`.

## Project goals

- Fast terminal workflow for Fossil checkouts
- Working tree and history browsing in one UI
- Commit subsets of files without a staging area
- Small, practical MVP first; polish later

## Current features

- Fossil checkout detection
- Working tree file list
- Diff / details pane
- Timeline / history view
- Temporary commit selection with `Space`
- Commit selected files, current file, or all files
- Keyboard and mouse navigation

## Commit flow

Fossil does not use a staging area.
Instead, lazyfossil builds commit commands like:

```bash
fossil commit file1 file2 file3 -m "commit message"
```

Extra files are added automatically before commit when needed.

## Roadmap

### Done
- Working tree MVP
- History timeline basics
- Temporary selection-based commit flow
- Inline commit message prompt

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