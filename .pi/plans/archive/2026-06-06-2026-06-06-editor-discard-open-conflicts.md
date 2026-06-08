---
title: "Add editor, discard, open-file, and conflict UI actions"
status: draft
created: "2026-06-06T09:03:30.121Z"
type: feature
---

## Goal
Add file actions and file-state UI updates to lazyfossil: edit in $EDITOR, discard changes, open files with external apps, rename the extra-file prefix, remove the current-file `>` marker, and surface merge conflicts in the file list.

## Features
- `e`: spawn `$EDITOR` on the current/selected file
- `d`: discard modification of the current file
- `o`: open the selected file using an application based on file extension
- show extra files with `??` instead of `E`
- highlight the current file by background instead of a `>` prefix
- show merge-conflict state in the file list

## Phases
### Phase 1 — File action plumbing
- Add Fossil helpers for editing/opening/discarding file state where possible.
- Define how `$EDITOR` is resolved and invoked.
- Define extension-based open rules for `o`.

### Phase 2 — Keybindings and behavior
- Bind `e`, `d`, and `o` in the TUI.
- Make actions operate on the current selected file.
- Handle command errors cleanly in the UI.

### Phase 3 — File list rendering
- Replace `E` with `??` for extra files.
- Remove the `>` marker and use background selection styling.
- Add merge-conflict detection and display a distinct conflict indicator.

### Phase 4 — Validation
- Add/update tests for status parsing and action dispatch.
- Verify manual flows in a Fossil checkout with edited, extra, and conflicted files.

## Verification
- Build and run tests.
- Manual smoke test:
  - `e` opens the selected file in `$EDITOR`
  - `d` reverts the current file
  - `o` opens files by extension
  - extra files show `??`
  - conflicted files are visible in the list
  - current file uses highlight only
