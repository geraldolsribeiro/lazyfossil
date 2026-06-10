---
title: "Add CI checks workflow and improve release pipeline"
status: draft
created: "2026-06-10T08:32:44.575Z"
type: feature
---

## Goal
Add a dedicated CI workflow for pull requests and pushes, and improve the release workflow so lazyfossil gets stronger validation before publishing tagged releases.

## Phases

### Phase 1 — Design the CI split
- Keep `.github/workflows/release.yml` focused on tag releases only.
- Add a new CI workflow for `push` and `pull_request`.
- Define the required checks:
  - `cargo fmt --all -- --check`
  - `cargo clippy -- -D warnings`
  - `cargo build --verbose`
  - `cargo test --verbose`

⏸️ Pause for review before editing files.

### Phase 2 — Improve release pipeline
- Update the release workflow to use explicit permissions.
- Standardize artifact naming for Linux, macOS, and Windows.
- Keep release uploads limited to tag builds.
- Consider whether additional targets (for example musl or ARM) should be added later.

### Phase 3 — Verification
- Run workflow syntax review mentally against the current repository layout.
- Confirm binary names match the package output.
- Confirm release assets are uploaded from the correct paths.

## Verification steps
- Check that PRs run format/lint/build/test.
- Check that tagged releases build and publish assets only on `v*` tags.
- Check that uploaded asset names are consistent and user-friendly.