---
title: "Implement roadmap items one at a time with user approval gates"
status: done
created: "2026-06-10T12:06:25.689Z"
type: feature
---

## Goal
Implement the roadmap items one by one, pausing after each item so the user can test and approve before moving to the next.

## Phases

### Phase 1 — UI polish
- Dynamic Details pane titles
- Preview styling per file type
- Footer and hint cleanup
- Spacing and layout tweaks

Verification:
- `cargo test --quiet`
- manual UI check in Working tree, File history, and Timeline tabs

⏸️ Pause and ask the user to test/approve before proceeding.

### Phase 2 — History and details
- Improve file history browsing
- Better timeline entry details
- Clearer commit metadata display
- Refine selected-file history behavior

Verification:
- `cargo test --quiet`
- manual review of history/timeline panes

⏸️ Pause and ask the user to test/approve before proceeding.

### Phase 3 — Status and recovery UX
- Better missing-file recovery
- Rename detection improvements
- Conflict state polish
- Clearer warnings and errors

Verification:
- `cargo test --quiet`
- manual checks for missing/rename/conflict flows

⏸️ Pause and ask the user to test/approve before proceeding.

### Phase 4 — Navigation and mouse
- Smoother scrolling
- Better mouse selection
- Improved pane focus behavior
- Edge-case fixes in selection

Verification:
- `cargo test --quiet`
- manual navigation checks

⏸️ Pause and ask the user to test/approve before proceeding.

### Phase 5 — Quality and tests
- Status parsing tests
- Preview rendering tests
- Regression tests for styling
- Coverage for missing, rename, and conflict cases

Verification:
- `cargo test --quiet`
- targeted test runs as needed

⏸️ Pause and ask the user to test/approve before proceeding.

### Phase 6 — Follow-up refinements
- Workflow tweaks discovered during use
- Small command/help improvements
- Docs updates as features change

Verification:
- `cargo test --quiet`
- docs consistency check

⏸️ Pause and ask the user to test/approve before proceeding.
