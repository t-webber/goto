$x = Invoke-Expression "gt.ps1 -get $args"
Write-Host "explorer.exe $x"
explorer.exe $x