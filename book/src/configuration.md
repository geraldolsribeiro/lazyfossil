# Configuration

lazyfossil supports an optional XDG-style config file:

```text
$XDG_CONFIG_HOME/lazyfossil/config.toml
```

If `XDG_CONFIG_HOME` is not set, it falls back to:

```text
~/.config/lazyfossil/config.toml
```

## Supported settings

- `editor`: default editor used for text files when `$EDITOR` is not set
- `binary_glob`: file glob passed to Fossil before committing binary files

## Example

```toml
editor = "nvim"
binary_glob = "*.png,*.jpg,*.jpeg,*.gif,*.ico,*.pdf"
```

## Notes

- `EDITOR` still takes priority when launching the editor from the terminal.
- If no config file exists, lazyfossil uses built-in defaults.
- You can keep configuration minimal and only set the values you need.
