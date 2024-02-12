$path=$(Invoke-Expression "gt.ps1 $args[0] $args[1] -get")
$path = $path -replace '\\', '/'
$path = '/mnt/' + $path -replace ':', '' -replace 'files', 'Files'
Code.exe --remote wsl+Debian $path