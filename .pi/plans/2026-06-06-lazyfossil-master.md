---
title: "lazyfossil master plan"
status: done
created: "2026-06-06T00:00:00.000Z"
type: feature
---

## Goal
Track the full lazyfossil MVP and follow-up work in one consolidated plan.

## Summary
- Rust TUI MVP for Fossil SCM
- Working tree browsing and diffs
- History timeline and file-history view
- Temporary selection-based commit flow
- Ignore and sync support
- Binary preview handling
- File actions: edit, discard, open externally
- Conflict handling and missing-file UX
- README/assets/release polish

## Completed areas
### Working tree
- Repo detection
- File list and selection
- Diff preview and extra-file preview
- Mouse selection and scrolling
- Tab expansion for text previews

### History
- Timeline entries
- Selected-path timeline refresh

### Commit flow
- Temporary selection with `Space`
- Commit selected/current/all files
- Extra-file add-then-commit flow
- Binary-file handling before commit

### Repo hygiene
- Ignore-file editing
- Sync command
- Command logging
- Binary preview fallback

### Polish
- Footer/input visibility
- README updates
- Logo assets
- Credits section
- Release workflow fix

## Remaining follow-ups
- Commit details pane
- Better history/file history browsing
- UI polish as needed
- Optional: rename/missing-file recovery improvements
- Optional: more tests around status parsing and message styling

## Status
Done