---
title: "Epic 1 timeline breakdown for successive refinement"
status: draft
created: "2026-06-08T17:27:59.939Z"
type: feature
---

## Goal
Break Epic 1 (Timeline as the signature feature) into small, shippable increments with clear acceptance criteria.

## Scope
Focus on the timeline experience first because it should become the main reason users open lazyfossil.

## Increment 1 — Readable timeline list
### Outcome
Users can scan timeline entries quickly without extra interpretation.

### Tasks
- Display timeline rows with consistent visual hierarchy
- Add clear author/date/message formatting
- Keep branch/merge indicators readable in compact width
- Ensure selected entry is obvious

### Acceptance Criteria
- Timeline entries are easy to distinguish at a glance
- The selected entry is visually clear
- No timeline row wraps awkwardly in normal terminal sizes

## Increment 2 — File-scoped timeline navigation
### Outcome
Users can inspect history for the current file directly from the working tree.

### Tasks
- Keep selected-file timeline in sync with file selection
- Refresh timeline when selection changes
- Show a helpful empty state when no history exists
- Make subfolder path handling reliable

### Acceptance Criteria
- Changing file selection updates the timeline view
- Path-scoped history works in subfolders
- Empty timeline states are clear and non-blocking

## Increment 3 — Timeline to diff/details jump
### Outcome
Users can move from history context to file details quickly.

### Tasks
- Jump from a timeline entry to the corresponding diff/details view
- Preserve selection and scroll state where practical
- Make the keyboard path obvious

### Acceptance Criteria
- A user can select a timeline item and inspect its diff/details
- Returning to timeline does not lose context unnecessarily

## Increment 4 — Timeline filtering and scope controls
### Outcome
Users can narrow the timeline to what matters now.

### Tasks
- Add basic timeline filters
- Add scope controls for path/branch context when possible
- Keep controls discoverable but unobtrusive

### Acceptance Criteria
- Users can reduce timeline noise without leaving the app
- Filters are understandable and reversible

## Increment 5 — Branch/merge context visualization
### Outcome
Timeline becomes Fossil-native and visually distinctive.

### Tasks
- Add visual hints for merges, branches, and tags
- Improve branch boundary readability
- Surface merge context in a compact way

### Acceptance Criteria
- Branch and merge context are visible without reading raw commands
- Timeline feels more Fossil-specific than a generic commit list

## Increment 6 — Timeline polish and tests
### Outcome
The timeline experience is reliable enough to become a signature feature.

### Tasks
- Add tests for parsing and selection behavior
- Add tests for filter/scope edge cases
- Polish labels, hints, and empty states

### Acceptance Criteria
- Timeline behavior is covered by tests
- Common edge cases do not break the view
- The feature is ready to build upon in later epics

## Build Order
1. Readable timeline list
2. File-scoped timeline navigation
3. Timeline to diff/details jump
4. Timeline filtering and scope controls
5. Branch/merge context visualization
6. Timeline polish and tests

## Notes
- Keep each increment shippable
- Prefer clarity over complex graph rendering in the first pass
- Add depth only after the base timeline is pleasant to use
