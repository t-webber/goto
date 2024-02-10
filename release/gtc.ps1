param(
    [string]$Folder,
    [string]$SubPath, 
    [Alias("w")]
    [switch]$wsl
)

if ($wsl) {
   $path= $(gt.ps1 get $Folder $Subpath )
   $path = $path -replace '\\', '/'
   $path = '/mnt/' + $path -replace ':', '' -replace 'files', 'Files'
   Code.exe --remote wsl+Debian $path | Out-Null
} elseif (!$Folder) {
    $Path = gt.ps1 get pwsh 
} else {
    $Path = gt.ps1 get $Folder $SubPath
}
& Code.exe "$Path" | Out-Null
return
