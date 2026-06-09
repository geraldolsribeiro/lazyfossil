Source terminal_config.tape

# Create a fresh Fossil checkout
Hide
Type "cd /tmp"
Enter
Type "rm -rf repo01.fossil work"
Enter
Type clear
Enter
Show

Type "fossil init repo01.fossil"
Enter
Sleep 2s

Type "mkdir -p work"
Enter
Sleep 1s

Type "cd work"
Enter
Sleep 1s

Type "fossil open ../repo01.fossil"
Enter
Sleep 1s

# Open lazyfossil into the fresh repository
Type lazyfossil
Sleep 2s
Enter
Sleep 5s
Type q

# Create a new file for the commit demo
Type "mkdir -p src"
Enter
Sleep 2s

Type "echo 'int main() { printf( hello world ); return 0; }' > src/my_file.c"
Enter
Sleep 2s

# Re-open lazyfossil to commit the file
Type lazyfossil
Sleep 1s
Enter
Sleep 2s

# Select the file
Space
Sleep 2s

# Start commit
Type c

# Enter the commit message
Type "First commit on my amazing project"
Sleep 1s

# Confirm commit
Enter
Sleep 5s
Type q
