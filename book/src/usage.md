# Usage

Fossil does not use a staging area like git does.
Instead, lazyfossil builds commit commands like:

```bash
fossil commit -m "commit message" file1 file2 file3
```

Extra files are added automatically before commit when needed.

Binary files are handled by setting:

```bash
fossil settings binary-glob "*.png,*.jpg,*.jpeg,*.gif,*.ico"
```

## Keybinds

### Navigation

- `Up` / `Down`: move between files
- `Tab`: switch between Working tree and History
- Mouse click: select a file
- Mouse wheel: scroll the diff/details pane

### File actions

- `Space`: toggle the selected file for commit
- `e`: open the current file in `$EDITOR`
- `o`: open the current file in the default program for its file type
- `d`: discard the current file
- `i`: add the current file to `.fossil-settings/ignore-glob`

### Commit and sync

- `c`: commit selected files
- `f`: commit the current file
- `a`: commit all files
- `p` / `P`: sync with the remote

### General

- `r`: refresh
- `q`: quit
