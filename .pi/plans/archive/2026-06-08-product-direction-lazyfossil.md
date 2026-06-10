---
title: "Transform lazyfossil from MVP into a Fossil daily driver"
status: draft
created: "2026-06-08T17:07:48.912Z"
type: feature
---

## Goal
Evolve lazyfossil from an MVP into a polished, opinionated daily driver for Fossil users. The product should feel native to Fossil workflows instead of imitating lazygit.

## Direction
- Optimize for Fossil-first workflows and terminology
- Reduce friction for common daily tasks
- Keep the UI fast, predictable, and low-noise
- Make repository state, file actions, and sync/commit flows obvious
- Prefer domain-specific UX over generic Git TUI patterns

## Phases
### Phase 1 — Product positioning
- Define the core daily-driver workflows lazyfossil should excel at
- Write down what the app should intentionally not copy from lazygit
- Align UI language with Fossil concepts

### Phase 2 — Workflow refinement
- Tighten commit, sync, ignore, discard, and history flows
- Improve missing-file, conflict, rename, and binary-file handling
- Make repository-root and subfolder behavior feel seamless

### Phase 3 — Information design
- Rework the layout to emphasize the most useful Fossil actions and state
- Improve file list density, details pane clarity, and status messaging
- Surface actionable hints without clutter

### Phase 4 — Polish and trust
- Improve keyboard discovery and shortcut consistency
- Add more tests around status parsing, message rendering, and workflow actions
- Update README/site copy to describe the product vision clearly

## Verification
- Smoke test the core daily workflows in a real Fossil checkout
- Confirm the UI is understandable without prior lazygit expectations
- Build and test after each major workflow/UI change

## Notes
- The product goal is not “feature parity with lazygit”
- The product goal is “best Fossil TUI for everyday use”
