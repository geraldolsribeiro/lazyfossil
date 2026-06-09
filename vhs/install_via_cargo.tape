Source terminal_config.tape

# Clear any previous install output
Hide
Type "cargo uninstall lazyfossil"
Enter
Sleep 4s
Type clear
Enter
Show

# Install from crates.io
Type "cargo install lazyfossil"
Sleep 1s
Enter
Sleep 20s

# Launch the app
Type "lazyfossil"
Sleep 2s
Enter
Sleep 1s
Down 20
Sleep 5s
Type q
Sleep 2s
