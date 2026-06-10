---
title: "Lazyfossil consolidated roadmap and reprioritization"
status: draft
created: "2026-06-10T14:15:32.469Z"
type: chore
---

# Goal
Consolidate the remaining pending plans into one prioritized roadmap so follow-up work is easier to execute and archive.

## Priority order
1. Release and workflow hygiene
2. Timeline rendering and metadata polish
3. UI/docs consistency cleanup
4. Lower-priority product-direction ideas

## Phases

### Phase 1 — Release and workflow hygiene
- Review and merge CI/release workflow follow-ups
- Keep release-only behavior on tags
- Preserve artifact naming and permissions consistency

Verification:
- `cargo test --quiet`
- workflow file review

⏸️ Pause after the workflow pass if any behavior changes are required.

### Phase 2 — Timeline rendering and metadata polish
- Keep timeline rows in hash → tag → message order
- Ensure tag styling remains readable
- Review timeline detail layout and commit metadata display
- Fold any remaining timeline spec/increment notes into the main code path

Verification:
- `cargo test --quiet`
- manual UI review of Timeline and File history tabs

⏸️ Pause for user approval before moving on.

### Phase 3 — UI and docs consistency cleanup
- Keep README and book wording aligned with behavior
- Remove duplicate or conflicting keybinding/help text
- Update docs when UI hints change
- Ensure version references stay consistent

Verification:
- `cargo test --quiet`
- docs scan for duplicated or stale wording

⏸️ Pause for approval before any doc-only changes are finalized.

### Phase 4 — Lower-priority product-direction ideas
- Revisit product-roadmap / direction drafts if still relevant
- Split any new work into smaller actionable plans
- Archive superseded drafts after consolidation

Verification:
- plan review only

⏸️ Pause for approval before creating new execution plans.

## Archive plan
- Treat older pending drafts as superseded once the consolidated roadmap is accepted
- Keep the active plan list short and execution-focused
