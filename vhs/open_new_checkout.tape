Set Shell bash
Set FontSize 15
Set Padding 30

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

Type "# Open lazyfossil into fresh repository"
Enter
Type lazyfossil
Sleep 2s
Enter
Sleep 5s
Type q

Type "# Creating a file in src folder"
Enter
Type "mkdir -p src"
Sleep 2s
Enter

Type "echo 'int main() { printf( hello world ); return 0; }' > src/my_file.c"
Sleep 2s
Enter

Type "# The new file commited by pressing space (select) and 'c' (commit)"
Enter
Sleep 2s
Type lazyfossil
Sleep 1s
Enter
Sleep 2s

# Select file
Space
Sleep 2s

# Start commit
Type c

Type "First commit on my amazing project"
Sleep 1s

# Confirm commit
Sleep 2s
Enter

Sleep 5s
Type q
