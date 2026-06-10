Source terminal_config.tape

Type lazyfossil
Enter

# Go to README in Files tab
Down@100ms 15
Sleep 2s

# Go to File History tab
Tab

# Navigate into commit
Down@300ms 20
Sleep 2s
Up@300ms 10
Sleep 2s

# Scroll the commit diff
PageDown@100ms 20
Sleep 2s
Screenshot file_history.png

# Quit
Type q
