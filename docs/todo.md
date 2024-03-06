//ADD: Print Help, Version, Errors
//ADD: when renaming a folder, changes all occurrences in dirs and hist.

## Bugs and undesired behaviours

//BUG: call on `-code` outputs 4 results => no cd implied.
//BUG: No arguments => no output: travels home.
//BUG: no argument travels home
//BUG: code opens file if path not exists

## Corrections

//ADD: Subpath incorrect
//ADD: Levenshtein distance on shortcuts ?

## Hist

//ADD: poplocal
//ADD: delete old items in hist stack
//BUG: push only dirs, not files !

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
//ADD: Open localhosts

## Other plateforms

//ADD: Supported in cmd
//ADD: enable traveling through ssh (it already exists but it is not very user friendly)

## Not decided Behaviours

//NB: del deletes only the path once.
//NB: `d:/files/dev/powershell/cmdlets;edit;pwsh;0` and `g -edit edit d:/files/dev/rust/goto` => `d:/files/dev/rust/goto;edit;pwsh;0`! Is it good ? I think so (the shortcut is meant to mean the folder, not its content ?).

## Interactions

//ADD: Shortcut does not exist, do you want to create it ?
//ADD: Shortcut already exists, do you want to overwrite it ?
//ADD: gui interface for when it is run by executable.
