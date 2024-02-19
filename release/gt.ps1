if (!$PSScriptRoot) { Write-Error "Goto cannot be used as executable, as it needs to be run in the current process." ; return }

$res = Invoke-Expression "$PSScriptRoot\goto.exe $args" 
$res = $res -split '#'


if ($res.length -gt 3 -and $res[0] -ne "0") {
  return $res
}

if ($res[1] -eq "1") {
  return $res[2] 
}

if ($res[0] -eq "1") {
  return
}

Set-Location $res[2]
