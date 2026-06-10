---
title: "Lazyfossil epic backlog with successive refinement"
status: draft
created: "2026-06-08T17:22:31.511Z"
type: feature
---

## Goal
Break the master product plan into an epic backlog ordered for successive refinement: improve the core experience in layers, with each pass making lazyfossil more Fossil-native, reliable, and useful as a daily driver.

## Guiding Strategy
- Ship the smallest useful improvement first
- Refine the same workflow multiple times rather than scattering effort
- Start with clarity, then speed, then depth, then polish
- Keep every step shippable and testable

## Epic 1 — Timeline as the signature feature
### Outcome
Make timeline navigation the main reason to open lazyfossil.

### Tasks
- Show timeline entries with clearer branch/merge context
- Improve timeline readability in the details pane
- Add filtering and path-scoped timeline modes
- Jump from timeline entry to diff/details
- Add visual hints for merges, tags, and branch boundaries
- Add tests for timeline parsing and selection behavior

### Refinement steps
1. Make timeline text more readable
2. Add navigation between timeline and file details
3. Add filtering and scope controls
4. Add branch/merge context visualization

## Epic 2 — Working tree clarity
### Outcome
Users can instantly understand repository state.

### Tasks
- Refine file status labels and colors
- Make missing/conflict/extra/edited states obvious
- Add repository summary counts
- Improve selected-row and selected-file visual emphasis
- Clarify details pane messaging for missing/binary/empty files
- Add tests for file-status parsing and rendering

### Refinement steps
1. Improve status labels and color contrast
2. Add summary counts and stronger hierarchy
3. Refine missing/conflict/binary messaging
4. Polish empty/no-diff states

## Epic 3 — Commit and selection flow
### Outcome
Commit work is fast, explicit, and Fossil-correct.

### Tasks
- Keep selected/current/all commit paths obvious
- Make select-all / select-none behavior predictable
- Improve bulk file selection UX
- Clarify extra-file add-then-commit behavior
- Add commit helper hints in the footer
- Add tests around commit path collection and selection toggles

### Refinement steps
1. Stabilize selection behavior
2. Clarify commit target language
3. Improve extra-file handling
4. Add footer guidance and tests

## Epic 4 — Sync, health, and trust
### Outcome
Users can trust lazyfossil for daily repository maintenance.

### Tasks
- Make sync status and results more visible
- Add repository health indicators
- Surface incoming/outgoing/unpublished check-ins
- Keep error messages actionable and consistent
- Refresh views immediately after sync
- Add tests for sync flow and error handling

### Refinement steps
1. Improve sync action feedback
2. Add health summary view
3. Show unpublished/outgoing/incoming state
4. Polish error wording and recovery hints

## Epic 5 — Fossil-native actions
### Outcome
The app feels designed for Fossil, not adapted from Git.

### Tasks
- Strengthen branch and tag workflows
- Add clearer artifact and check-in inspection
- Improve external diff/editor integration
- Keep repository-root/subfolder behavior seamless
- Add contextual help for Fossil concepts
- Add tests for path handling and action dispatch

### Refinement steps
1. Improve editor/open/diff handoff
2. Add help for Fossil-specific terminology
3. Refine path handling in subfolders
4. Expand branch/tag/artifact actions

## Epic 6 — Configuration minimalism
### Outcome
Users can install and start working immediately.

### Tasks
- Audit configuration surface area
- Provide sensible defaults for common setups
- Avoid requiring theme/config files for basic use
- Document zero-config workflows
- Add tests for default behavior where practical

### Refinement steps
1. Remove unnecessary knobs
2. Improve defaults
3. Document the no-config path
4. Keep future settings optional

## Epic 7 — Documentation and discoverability
### Outcome
People can find, install, and understand lazyfossil quickly.

### Tasks
- Improve README onboarding
- Add screenshots and GIF/demo assets
- Write concise release notes
- Explain Fossil-specific workflows clearly
- Add contextual help copy in the UI
- Keep packaging and install docs current

### Refinement steps
1. Improve onboarding copy
2. Add visual demos
3. Tighten release notes
4. Update help text alongside features

## Epic 8 — Read-only ecosystem views
### Outcome
lazyfossil expands into a repository cockpit.

### Tasks
- Add ticket browsing
- Add wiki browsing
- Add forum reading
- Add technote viewing
- Keep these views read-only at first
- Add tests for new content adapters and navigation

### Refinement steps
1. Prototype read-only viewing
2. Add navigation and filtering
3. Improve cross-linking between repository views
4. Consider editing actions later

## Suggested Order
1. Timeline as the signature feature
2. Working tree clarity
3. Commit and selection flow
4. Sync, health, and trust
5. Fossil-native actions
6. Configuration minimalism
7. Documentation and discoverability
8. Read-only ecosystem views

## Success Criteria
- Each pass improves the same core workflows
- The app becomes more Fossil-native after every iteration
- Timeline and working tree become the two strongest reasons to use it
- The UI stays simple while gaining depth
