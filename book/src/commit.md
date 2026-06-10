# Commit

lazyfossil keeps the commit flow simple: mark the files you want, start the commit, type a message, and confirm.

## Example

1. Mark a file with `Space`
2. Press `c`
3. Type a short message
4. Press `Enter`

That is usually enough for a normal commit.

![commit demo](https://pub-0503d20ece60405d98e4a2fa8b21779d.r2.dev/commit.gif)

## Mark files for commit

In the **Working tree** view:

- use `Space` to mark or unmark the selected file
- use `a` to select all files or clear the selection
- use `c` to start a commit for the marked files
- use `f` to commit only the current file

The selected files are shown in the footer so you can check what will be included.

## Start the commit

Press `c` to open the commit prompt for the files you marked.
If nothing is marked, lazyfossil uses the currently selected file.

## Write the commit message

Type the message directly in the prompt.
Use:

- `Backspace` to edit
- `Esc` to cancel
- `Enter` to confirm

The message must not be empty.

## What happens with extra files

If you include extra files in the commit selection, lazyfossil adds them before it runs the final commit command.
That means you can select a new file, commit it, and let lazyfossil prepare it for Fossil automatically.

## Binary files

Binary files are handled by setting:

```bash
fossil settings binary-glob "*.png,*.jpg,*.jpeg,*.gif,*.ico"
```

You can override this in the XDG config file if needed.

