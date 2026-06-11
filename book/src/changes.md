# Changes

The **Changes** tab shows the non-clean files in the checkout:

- edited files
- added files
- deleted files
- missing files
- extra files
- conflict files

Use it to review only the files that need attention.

![Changes demo](https://pub-0503d20ece60405d98e4a2fa8b21779d.r2.dev/changes.gif)

## How it works

- It starts as the first tab in lazyfossil.
- The selected file stays in sync with the working tree.
- If the selected file is not part of the Changes list, the first visible change is selected.
- If there are no changes, nothing is selected.
- The selected entry is centered when possible so the list stays easy to scan.

## Notes

- Changes is a filtered view of the working tree.
- Use the Working tree tab when you want to see clean files too.
- Discard, commit, and open actions still apply to the currently selected file.
