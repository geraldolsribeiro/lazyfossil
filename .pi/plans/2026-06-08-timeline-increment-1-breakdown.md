---
title: "Timeline increment 1 task breakdown"
status: draft
created: "2026-06-08T17:29:11.608Z"
type: feature
---

## Goal
Implement Increment 1 of Epic 1: make the timeline list readable and easy to scan.

## Outcome
Users can quickly understand the timeline without extra interpretation.

## Workstreams

### 1. Data formatting
- Review the timeline data currently exposed to the UI
- Decide which fields should be shown in the compact list row
- Normalize author/date/message formatting for consistency
- Truncate or wrap fields intentionally so rows stay readable

### 2. UI hierarchy
- Give the selected timeline row a clear visual treatment
- Separate author, date, and message with a consistent structure
- Keep the list dense enough for real usage but not cramped
- Ensure the default empty state is understandable

### 3. Fossil-specific hints
- Make branch/merge indicators visible but compact
- Prefer Fossil terminology over generic commit wording
- Avoid overloading each row with too much metadata

### 4. Responsive layout
- Verify the timeline remains readable in narrow terminals
- Prevent awkward wrapping or clipped important text
- Keep the layout stable alongside the working tree pane

### 5. Tests
- Add tests for any timeline formatting helpers
- Add tests for selection behavior if display changes affect it
- Add tests for empty-state rendering where practical

## Suggested Task Order
1. Define the desired row layout
2. Implement the list formatting changes
3. Improve selected-row emphasis
4. Validate narrow-terminal behavior
5. Add tests

## Acceptance Criteria
- Timeline entries are easy to distinguish at a glance
- Selected entry is visually obvious
- Rows remain legible at normal terminal sizes
- The implementation is covered by tests where appropriate

## Notes
- Favor incremental improvement over a full graph rewrite
- Keep this pass focused on scanability and hierarchy
- Deeper branch/merge visualization belongs in a later increment
