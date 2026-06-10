# lazyfossil

[![Crates.io](https://img.shields.io/crates/v/lazyfossil)](https://crates.io/crates/lazyfossil)
[![Multiplatform Release](https://github.com/geraldolsribeiro/lazyfossil/actions/workflows/release.yml/badge.svg)](https://github.com/geraldolsribeiro/lazyfossil/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![GitHub stars](https://img.shields.io/github/stars/geraldolsribeiro/lazyfossil)](https://github.com/geraldolsribeiro/lazyfossil/stargazers)
![Crates.io Total Downloads](https://img.shields.io/crates/d/lazyfossil)

![lazyfossil logo](https://pub-0503d20ece60405d98e4a2fa8b21779d.r2.dev/lazyfossil_logo.png)

## Project goals

**lazyfossil** helps you work with **Fossil SCM** from the terminal in a simpler way:
browse files, see history, preview changes, and commit or sync without extra friction.

* <https://lazyfossil.org>
* <https://chiselapp.com/user/geraldo/repository/lazyfossil/>
* <https://github.com/geraldolsribeiro/lazyfossil>
* <https://crates.io/crates/lazyfossil>


- Provide a fast terminal workflow for Fossil checkouts
- Combine working-tree and history browsing in a single UI
- Commit subsets of files without a staging area
- Start with a small, practical MVP and refine it over time
- Show a clear "not in a checkout" state when launched outside Fossil

## Quick start

1. Install it with `cargo install lazyfossil`
2. Go to your project checkout
3. Type `lazyfossil`
4. Browse changes and history
5. Press `q` to quit

## Installation

## From crates.io

This is the preferred way to install and update **lazyfossil**.

```bash
cargo install lazyfossil
```

Follow instructions in <https://rust-lang.org/tools/install/> to install
**rust**.

## Pre-built binaries

Download pre-built binaries from the latest [GitHub releases](https://github.com/geraldolsribeiro/lazyfossil/releases/):

- [Linux](<https://github.com/geraldolsribeiro/lazyfossil/releases/latest/download/lazyfossil-linux>)
- [macOS](<https://github.com/geraldolsribeiro/lazyfossil/releases/latest/download/lazyfossil-macos>)
- [Windows](<https://github.com/geraldolsribeiro/lazyfossil/releases/latest/download/lazyfossil-windows.exe>)

## Source code

The lazyfossil [source
code](https://chiselapp.com/user/geraldo/repository/lazyfossil/) is distributed under MIT license, and is maintained
via [Fossil SCM](https://fossil-scm.org/).

There are mirror repositories at 
[GitHub](https://github.com/geraldolsribeiro/lazyfossil) and
[crates.io](https://crates.io/crates/lazyfossil).

## Screenshots and demos

**lazyfossil** is still in alpha, but it is already useful for everyday Fossil work. This project uses lazyfossil itself while we continue refining it, so the screenshots and demos reflect real use rather than a polished finish. If you work with Fossil from the terminal, try it and see whether it fits your workflow.

lazyfossil’s main screen is built around three views (Working tree, File history, and Timeline):

![working tree demo](https://pub-0503d20ece60405d98e4a2fa8b21779d.r2.dev/working_tree.gif)

![File history demo](https://pub-0503d20ece60405d98e4a2fa8b21779d.r2.dev/file_history.gif)

![Timeline demo](https://pub-0503d20ece60405d98e4a2fa8b21779d.r2.dev/timeline.gif)

## Why use lazyfossil
- Keep Fossil work inside a fast terminal UI
- See files, history, and diffs without leaving the app
- Commit selected files or the current file, without a staging area
- Handle sync, ignore, and discard tasks from one place
- Stay productive even for binary files and missing-file cases

## What it gives you
- A clear view of your working tree and extras
- File history and repository timeline in the same interface
- Helpful previews for text, binary, and hex views
- Simple keyboard and mouse navigation, including left-pane selection scrolling
- Quick commit and sync actions with confirmation prompts
- Toggle all/none selection with `a`
- Better handling for common Fossil workflows

## Roadmap

### Done
- Working tree, file history, and timeline views
- Temporary selection-based commit flow
- Inline commit message prompt
- Ignore-file support
- Sync support
- Binary preview fallback
- XDG config file support
- Book documentation for the main views and config
- VHS demo tapes for the main flows

### Now
- UI polish and layout tweaks
- Better file history and timeline details
- Status and recovery UX

### Next
- Navigation and mouse
- Quality and tests

### Later
- Follow-up refinements
  - workflow tweaks discovered during use
  - small command/help improvements
  - docs updates as features change

## Versioning

This project follows semantic versioning: `MAJOR.MINOR.PATCH`.

Current version: `0.7.2`.

## Star History

[![Star History Chart](https://api.star-history.com/chart?repos=geraldolsribeiro/lazyfossil&type=date&legend=top-left)](https://www.star-history.com/?repos=geraldolsribeiro%2Flazyfossil&type=date&legend=top-left)

## Credits

### [fossil-scm.org](https://fossil-scm.org/)
The Fossil SCM project is the version control system lazyfossil is built around.

### [pi.dev](https://pi.dev)
Pi provides the agent harness used to develop and refine this project. Its tooling made it easier to iterate quickly, validate changes, and improve the TUI with confidence.

### [crates.io/crates/lazyfossil](https://crates.io/crates/lazyfossil)
The crates.io is the distribution channel for the Rust application, helping make lazyfossil available to the broader Rust ecosystem and simplifying installation and release management.

### [emojicombos.com/lazyfossil](https://emojicombos.com/lazyfossil)
This source provided the project logo artwork used in the README and assets, giving lazyfossil a recognizable visual identity.

## Other fossil companion tools

* fnc - <https://fnc.sh>
* diesel - <https://github.com/AnotherFoxGuy/diesel-scm>
* fuel - <https://fuel-scm.org/>
