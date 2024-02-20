//ADD: Print Help, Version, Errors
//ADD: Not supported in cmd
//ADD: when renaming a folder, changes all occurrences in dirs and hist.
//BUG: call on code outputs 4 results => no cd implied.

## Hist

//ADD: poplocal
//ADD: popbackuplocal
//ADD: popclearlocal
//ADD: delete old backups
//ADD: delete old items in hist stack

## Deletions

//ADD: start with priority 1000 and delete everybody with priority 0
//ADD: Backup

## Web

//ADD: Web option to open files
//ADD: List of websites with aliases

# Unspecified behaviours

//NOTE: del deletes only the path once.
//NOTE: `d:/files/dev/powershell/cmdlets;edit;pwsh;0` and `gt -edit edit d:/files/dev/rust/goto` => `d:/files/dev/rust/goto;edit;pwsh;0`! Is it good ? I think so (the shortcut is meant to mean the folder, not its content ?).
