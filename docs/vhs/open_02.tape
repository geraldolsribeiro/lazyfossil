Output open_02.gif
Set Shell bash
Set FontSize 15
Set Padding 30

Hide
Type "rm -rf repo01.fossil work"
Enter
Type clear
Enter
Show

Type whoami
Enter
Sleep 2s

Type "fossil init --user root repo01.fossil"
Enter
Sleep 2s

Type "mkdir -p work"
Enter
Sleep 2s

Type "cd work"
Enter
Sleep 2s

Type "fossil open ../repo01.fossil"
Enter
Sleep 2s

Type lazyfossil
Enter
Sleep 5s
Type q

Hide
Type "chown 1000:1000 *.gif"
Enter
