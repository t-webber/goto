$path=$(Invoke-Expression "gtexe  $args -get")
$path = $path -replace '\\', '/'
$path = '/mnt/' + $path -replace ':', ''
Invoke-Expression "Code.exe --remote wsl+Ubuntu $path"