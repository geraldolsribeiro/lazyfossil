# Hex preview

lazyfossil can show a hex dump for the selected file when text preview is not enough.

![hex preview demo](https://pub-0503d20ece60405d98e4a2fa8b21779d.r2.dev/hex_preview.gif)

## When to use it

Use hex preview for binary files or files whose content you want to inspect byte by byte.
It is especially useful when the normal diff view is not readable.

## How to open it

In the **Working tree** view, press `H` to toggle hex preview on or off.

When hex preview is active, the details pane shows offsets, hexadecimal bytes, and the printable ASCII view side by side.

## What happens for binary files

If lazyfossil detects that a file is binary, it shows a friendly preview notice first.
From there, you can:

- press `H` to inspect the file in hex
- press `o` to open it in an external program

## Notes

Hex preview is a viewing aid only. It does not change the file or the repository.
