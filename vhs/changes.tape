Source terminal_config.tape

Type lazyfossil
Enter

# Open the filtered Changes tab
Sleep 2s

# Move through the visible changes
Down@250ms 3
Up@250ms 1
Screenshot changes.png
Sleep 2s

Tab
# Show the centered selection and nearby files
PageDown@200ms 5
Sleep 2s

Tab
Sleep 2s
Down 3
Sleep 1s

Tab
Sleep 2s
Down 3
Sleep 1s

Tab
Sleep 2s
Down@1s 3
Up@1s 3

# Quit
Type q
