---
title: "Build a lazygit-like Fossil TUI MVP in Rust"
status: draft
created: "2026-06-02T19:59:56.272Z"
type: feature
---

## Goal
Create a Rust terminal application inspired by lazygit for Fossil SCM, focused on an MVP for working tree and history workflows.

## Current status
- Rust TUI scaffold is in place
- Working tree list + selected-file diff are working
- Mouse selection and diff scrolling have been added
- History view is present in basic form

## Scope
- Repository detection and basic app shell
- Working tree view: files, status, diff preview
- History view: commits/timeline and file history
- Core Fossil commands wrapped behind a thin service layer
- Keyboard navigation and a simple TUI layout

## Phases
### Phase 1 — Project scaffolding
- ✅ Initialize Rust TUI app structure
- ✅ Add command runner abstraction for Fossil CLI
- ✅ Define data models for status, diff, and timeline entries

### Phase 2 — Working tree MVP
- ✅ Show changed files with status
- ✅ Display selected file diff
- ✅ Refresh repository state from Fossil
- ✅ Add mouse selection for filenames
- ✅ Add diff scrolling

### Phase 3 — History MVP
- ✅ Show recent timeline entries
- ⏸️ View commit details and file history for selected path

### Phase 4 — Polish
- ✅ Keybindings, error handling, loading states
- ✅ Minimal styling and layout tuning
- ⏸️ Improve footer/status visibility and layout consistency
- ⏸️ Expand mouse interactions and scroll behavior

## Verification
- Run unit tests for command parsing/model conversion
- Manual smoke test against a sample Fossil repository
- Verify app starts, detects repo, shows status, diff, and history

## ⏸️ Pause points
- After working tree MVP, validate command coverage before history work
- Before committing, confirm explicitly with the user