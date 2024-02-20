# $in = $args[0]
# $inn = $args[1]

$path=$(Invoke-Expression "gtexe  $args -get")
$path = $path -replace '\\', '/'
$path = '/mnt/' + $path -replace ':', ''
# Write-Host "Remote: $path"
# Sleep 1
Invoke-Expression "Code.exe --remote wsl+Debian $path"