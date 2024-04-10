goto.exe $args | % { 
  if ($_ -eq "0") {
    $ChangeLocation = $true
  } elseif ($_ -ne "1" -and $_) {
    $Location += "$_`n"
  } 
}

if ($Location -eq $null) {
  return
}
$location = $Location.Trim()

if ($ChangeLocation -and $location -and (Test-Path $location)) {
  Set-Location $Location
} else {
  Write-Output $Location
}