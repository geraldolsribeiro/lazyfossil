---
title: "Lazyfossil product roadmap for Fossil daily driver"
status: draft
created: "2026-06-08T17:08:56.734Z"
type: feature
---

## Goal
Turn lazyfossil into the best daily-driver TUI for Fossil users.

## Product Principles
- Fossil-first, not Git-first
- Fast to scan, fast to act
- Make state obvious; keep noise low
- Favor clear domain terms over generic TUI patterns
- Default to safe actions with explicit confirmations when needed

## Must-Have Daily Workflows
### 1. Inspect working tree
- See tracked, extra, missing, edited, added, deleted, and conflicted files clearly
- Quickly understand what changed and what needs attention
- Preview text, binary, and hex content predictably

### 2. Commit work
- Select one, many, or all files
- Commit with clear path selection and extra-file handling
- Make staged/selected intent obvious in the UI
- Keep commit flow fast and low-friction

### 3. Sync with remote
- One-key sync action
- Refresh state immediately after sync
- Surface sync errors clearly

### 4. Manage local hygiene
- Ignore files intentionally, with confirmation
- Discard changes safely
- Open files in editor or external app
- Handle missing files and rename hints gracefully

### 5. Understand history
- File-scoped history browsing
- Timeline that works reliably in subfolders and repo roots
- Clear author/date/message presentation

## Non-Goals
- Recreating lazygit’s full feature set
- Copying Git-centric UI metaphors that don’t fit Fossil
- Complex staging model that obscures Fossil’s workflow
- Dense panels that hide the current task

## Roadmap Phases
### Phase A — Workflow clarity
- Refine file list, details pane, and footer messaging
- Standardize action labels and confirmations
- Improve missing/conflict/binary/rename messaging

### Phase B — Daily-driver speed
- Reduce keystrokes for common tasks
- Improve selection and bulk actions
- Make sync/commit flows feel immediate and trustworthy

### Phase C — State and history
- Polish file state parsing and visual hierarchy
- Improve timeline browsing and selected-file history
- Add helpful hints without clutter

### Phase D — Trust and polish
- Expand tests for parsing, actions, and message formatting
- Improve README/site positioning
- Add small quality-of-life touches based on real usage

## Initial Milestones
1. Clarify UI language and action labels
2. Simplify working tree scanning and selection flows
3. Improve missing/conflict/rename UX
4. Tighten history browsing and diff preview behavior
5. Add more test coverage for core workflows

## Success Criteria
- New Fossil users can understand the UI quickly
- Common tasks are doable in a few keystrokes
- The app feels like a Fossil companion, not a lazygit clone
- Users trust the app for everyday repository work
