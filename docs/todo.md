//ADD: Print Help, Version, Errors
//ADD: when renaming a folder, changes all occurrences in dirs and hist.
//ADD: Not supported in cmd
//BUG: call on `-code` outputs 4 results => no cd implied.

## Corrections

//ADD: Subpath incorrect
//ADD: Levenshtein distance on shortcuts ?
//ADD: priority

## Hist

//ADD: poplocal
//ADD: delete old items in hist stack

## Backups

//ADD: backup
//ADD: popclearlocal
//ADD: popbackuplocal
//ADD: delete old backups
//ADD: start with priority 1000 and delete everybody with priority 0
//ADD: decrementation on every call of goto, instead of on powershell launch

## Web

//ADD: Web option to open files
//ADD: List of websites with aliases

# Not decided Behaviours

//NB: del deletes only the path once.
//NB: `d:/files/dev/powershell/cmdlets;edit;pwsh;0` and `gt -edit edit d:/files/dev/rust/goto` => `d:/files/dev/rust/goto;edit;pwsh;0`! Is it good ? I think so (the shortcut is meant to mean the folder, not its content ?).
