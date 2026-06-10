# File history

The **File history** tab shows the commit history for the file currently selected in the working-tree list.

![File history](https://pub-0503d20ece60405d98e4a2fa8b21779d.r2.dev/file_history.gif)

## What it shows

- the selected file’s timeline entries from Fossil
- commit id, author, date, and message for each entry
- the full diff for the highlighted commit in the details pane

## How to use it

1. Open a Fossil checkout in lazyfossil.
2. Use the left pane to select a file.
3. Press `Tab` until you reach **File history**.
4. Move with `Up` and `Down`, or click an entry with the mouse.

The details pane updates to show:

- commit information at the top
- the full commit diff below it

## Notes

- File history is based on the selected working-tree file.
- Timeline mode is separate: it shows repository-wide history instead of file-specific history.
- Use the mouse wheel or `PageUp` / `PageDown` to scroll longer diffs.
