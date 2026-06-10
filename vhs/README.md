# VHS demos and media

This directory contains source tapes for reproducible terminal demos used in the README, release notes, website, and other docs.

## Suggested workflow

- Record terminal demos with [VHS](https://github.com/charmbracelet/vhs)
- Export screenshots or GIFs from the same scripted terminal sessions
- Keep assets aligned with the current release version
- From the repository root, run `make media` to build the release binary and render all tapes in `vhs/`

## Current demos

- `file-history.tape` — shows the file-history view with commit details and diffs
- `install_via_cargo.tape` — installs lazyfossil from crates.io and launches it
- `open_new_checkout.tape` — creates a fresh checkout, opens lazyfossil, and commits a new file
- `open_out_of_checkout_dir.tape` — shows the not-in-checkout startup state

## Conventions

- Keep demos short and focused on a single capability
- Prefer deterministic interactions and visible key presses
- Re-export screenshots/GIFs/videos from these tapes when the UI changes
- Store exported images/video in Cloudflare R2 bucket
- The base URL for images is `https://pub-0503d20ece60405d98e4a2fa8b21779d.r2.dev/lazyfossil/filename.gif`

## Theme

- Preview at <https://github.com/flemay/vhs-themes/tree/themes>
