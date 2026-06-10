---
title: "Timeline row rendering spec"
status: draft
created: "2026-06-08T17:31:20.866Z"
type: feature
---

## Goal
Implement the first timeline refinement by making timeline rows easy to scan and understand.

## Row Layout
- Line 1: `rid` + optional branch/tag hints + primary message
- Line 2: `author · date`

## Styling Rules
- `rid`: cyan/dim
- branch/tag hints: yellow/bold
- primary message: white/normal
- author/date: dark gray
- selected row: full-row highlight across both lines

## Behavior
- Keep rows compact and readable in normal terminal sizes
- Truncate or visually shorten long messages before layout breaks
- Keep the working tree pane stable alongside the timeline
- Show a clear empty state: `No timeline entries found`

## Implementation Tasks
1. Update timeline row rendering to two lines per entry
2. Add compact `rid` styling
3. Add branch/tag hint spans if available
4. Render message as the primary line
5. Render author/date as the secondary dim line
6. Apply selected-row highlight across both lines
7. Add empty-state text
8. Add tests for formatting/rendering helpers

## Acceptance Criteria
- Timeline entries are easy to distinguish at a glance
- Selected entry is visually obvious
- Rows remain legible at normal terminal sizes
- Tests cover formatting/rendering helpers where practical

## Notes
- Prefer incremental improvement over a graph rewrite
- Keep this pass focused on scanability and hierarchy
